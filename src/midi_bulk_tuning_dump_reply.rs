use crate::consts::{BULK_DUMP_REPLY, EOX, MIDI_TUNING, SYSEX, UNIVERSAL_NON_REAL_TIME};
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
pub(crate) struct MidiBulkTuningDumpReply {
    #[allow(unused)]
    device_id: u8,

    #[allow(unused)]
    program_number: u8,

    #[allow(unused)]
    name: String,

    #[allow(unused)]
    records: Vec<Record>,

    #[allow(unused)]
    checksum: u8,
}

impl MidiBulkTuningDumpReply {
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

impl MidiBulkTuningDumpReply {
    // https://midi.org/midi-tuning-updated-specification
    pub(crate) fn from_bytes<R: Read>(bytes: Bytes<R>) -> Result<Self> {
        let mut iter = bytes.filter_map(Result::<_, _>::ok).peekable();

        if read!(iter) != SYSEX {
            bail!("Unsupported header");
        }

        if read!(iter) != UNIVERSAL_NON_REAL_TIME {
            bail!("Unsupported header");
        }

        let device_id = read!(iter);

        if read!(iter) != MIDI_TUNING {
            bail!("Expected MIDI Tuning")
        }

        if read!(iter) != BULK_DUMP_REPLY {
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

        let checksum = read!(iter);

        if read!(iter) != 0xf7 {
            bail!("EOX not found");
        }

        Ok(Self {
            device_id,
            program_number,
            name,
            records,
            checksum,
        })
    }

    #[allow(unused)]
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        bytes.push(SYSEX);
        bytes.push(UNIVERSAL_NON_REAL_TIME);
        bytes.push(self.device_id);
        bytes.push(MIDI_TUNING);
        bytes.push(BULK_DUMP_REPLY);
        bytes.push(self.program_number);

        let name_bytes = self.name.as_bytes();
        let name_bytes_len = name_bytes.len();
        if name_bytes_len > 16 {
            bail!("Invalid name");
        }

        bytes.extend(name_bytes);
        for _ in 0..(16 - name_bytes_len) {
            bytes.push(0x00);
        }

        if self.records.len() != 128 {
            bail!("Invalid number of records");
        }

        for record in &self.records {
            bytes.push(record.xx);
            bytes.push(record.yy);
            bytes.push(record.zz);
        }

        bytes.push(self.checksum);
        bytes.push(EOX);

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use crate::midi_bulk_tuning_dump_reply::MidiBulkTuningDumpReply;
    use crate::resources::RESOURCE_DIR;
    use anyhow::{anyhow, Result};
    use std::io::Read;

    #[test]
    fn basics() -> Result<()> {
        let bytes = RESOURCE_DIR
            .get_file("syx/carlos_super.syx")
            .ok_or_else(|| anyhow!("Could not load tuning dump"))?
            .contents();
        let reply = MidiBulkTuningDumpReply::from_bytes(bytes.bytes())?;
        let output = reply.to_bytes()?;
        assert_eq!(bytes, output);
        Ok(())
    }
}
