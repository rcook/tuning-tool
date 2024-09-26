use anyhow::{bail, Result};
use std::io::{Bytes, Read};

macro_rules! read {
    ($iter: expr) => {
        $iter
            .next()
            .ok_or_else(|| ::anyhow::anyhow!("Failed to read byte"))?
    };
    ($iter: expr, $count: expr) => {{
        let mut result = Vec::with_capacity($count);
        for _ in 0..$count {
            result.push(
                $iter
                    .next()
                    .ok_or_else(|| ::anyhow::anyhow!("Failed to read byte"))?,
            );
        }
        result
    }};
}

#[derive(Debug)]
pub(crate) struct Record {
    #[allow(unused)]
    xx: u8,

    #[allow(unused)]
    yy: u8,

    #[allow(unused)]
    zz: u8,
}

#[derive(Debug)]
pub(crate) struct MidiTuningBulkDumpReply {
    #[allow(unused)]
    device_id: u8,

    #[allow(unused)]
    program_number: u8,

    #[allow(unused)]
    name: String,

    #[allow(unused)]
    records: Vec<Record>,
}

impl MidiTuningBulkDumpReply {
    #[allow(unused)]
    #[must_use]
    pub(crate) const fn device_id(&self) -> u8 {
        self.device_id
    }

    #[allow(unused)]
    #[must_use]
    pub(crate) const fn program_number(&self) -> u8 {
        self.program_number
    }
}

impl MidiTuningBulkDumpReply {
    // https://midi.org/midi-tuning-updated-specification
    pub(crate) fn from_bytes<R: Read>(bytes: Bytes<R>) -> Result<Self> {
        let mut iter = bytes.filter_map(Result::<_, _>::ok).peekable();

        if read!(iter) != 0xf0 {
            bail!("Unsupported header");
        }

        if read!(iter) != 0x7e {
            bail!("Unsupported header");
        }

        let device_id = read!(iter);

        if read!(iter) != 0x08 {
            bail!("Expected MIDI Tuning")
        }

        if read!(iter) != 0x01 {
            bail!("Expected Bulk Dump reply")
        }

        let program_number = read!(iter);

        let temp = String::from_utf8(read!(iter, 16))?;
        let name = String::from(temp.trim());

        let records = (0..128)
            .map(|_| {
                let xx = read!(iter);
                let yy = read!(iter);
                let zz = read!(iter);
                Ok(Record { xx, yy, zz })
            })
            .collect::<Result<Vec<_>>>()?;

        // Checksum: ignore
        _ = read!(iter);

        if read!(iter) != 0xf7 {
            bail!("EOX not found");
        }

        Ok(Self {
            device_id,
            program_number,
            name,
            records,
        })
    }
}
