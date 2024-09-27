use crate::midi::checksum_calculator::ChecksumCalculator;
use crate::midi::consts::{
    BULK_DUMP_REPLY, BULK_DUMP_REPLY_MESSAGE_SIZE, EOX, MIDI_TUNING, SYSEX, UNIVERSAL_NON_REAL_TIME,
};
use crate::midi::midi_frequency::MidiFrequency;
use crate::num::is_u7;
use anyhow::{bail, Result};
use std::io::{Bytes, Read};

macro_rules! read_u7 {
    ($iter: expr) => {{
        let b: u8 = read_u8!($iter);
        assert!(crate::num::is_u7(b));
        b
    }};
    ($iter: expr, $count: expr) => {{
        let mut result = Vec::with_capacity($count);
        for _ in 0..$count {
            result.push(read_u7!($iter));
        }
        result
    }};
}

macro_rules! read_u8 {
    ($iter: expr) => {{
        let b: u8 = $iter
            .next()
            .ok_or_else(|| ::anyhow::anyhow!("Failed to read u7"))?;
        b
    }};
    ($iter: expr, $count: expr) => {{
        let mut result = Vec::with_capacity($count);
        for _ in 0..$count {
            result.push(read_u8!($iter));
        }
        result
    }};
}

#[derive(Debug)]
pub(crate) struct BulkTuningDumpReply {
    #[allow(unused)]
    device_id: u8,

    #[allow(unused)]
    program_number: u8,

    #[allow(unused)]
    name: String,

    #[allow(unused)]
    frequencies: [MidiFrequency; 128],
}

impl BulkTuningDumpReply {
    pub(crate) fn new(
        device_id: u8,
        program_number: u8,
        name: &str,
        frequencies: [MidiFrequency; 128],
    ) -> Result<Self> {
        if !is_u7(device_id) {
            bail!("Invalid device ID");
        }
        if !is_u7(program_number) {
            bail!("Invalid program number");
        }
        if name.len() > 16 {
            bail!("Invalid name");
        }

        Ok(Self {
            device_id,
            program_number,
            name: String::from(name),
            frequencies,
        })
    }

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

impl BulkTuningDumpReply {
    // https://midi.org/midi-tuning-updated-specification
    pub(crate) fn from_bytes<R: Read>(bytes: Bytes<R>) -> Result<Self> {
        let mut calc = ChecksumCalculator::new(0xff);

        let mut iter = bytes.filter_map(Result::<_, _>::ok).peekable();

        if read_u8!(iter) != SYSEX {
            bail!("Unsupported header");
        }

        if calc.update(read_u7!(iter)) != UNIVERSAL_NON_REAL_TIME {
            bail!("Unsupported header");
        }

        let device_id = calc.update(read_u7!(iter));

        if calc.update(read_u7!(iter)) != MIDI_TUNING {
            bail!("Expected MIDI Tuning")
        }

        if calc.update(read_u7!(iter)) != BULK_DUMP_REPLY {
            bail!("Expected Bulk Dump reply")
        }

        let program_number = calc.update(read_u7!(iter));

        let name_bytes = read_u7!(iter, 16);

        for b in &name_bytes {
            _ = calc.update(*b)
        }

        let name = String::from(String::from_utf8(name_bytes)?.trim());

        let frequencies: [MidiFrequency; 128] = (0..128)
            .map(|_| {
                let xx = read_u7!(iter);
                let yy = read_u7!(iter);
                let zz = read_u7!(iter);
                Ok(MidiFrequency::new(xx, yy, zz)?)
            })
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .expect("Vector must have exactly 128 elements");

        for f in &frequencies {
            _ = calc.update(f.xx());
            _ = calc.update(f.yy());
            _ = calc.update(f.zz());
        }

        let checksum = read_u7!(iter);

        if read_u8!(iter) != EOX {
            bail!("EOX not found");
        }

        calc.verify(checksum, Some(BULK_DUMP_REPLY_MESSAGE_SIZE))?;

        Ok(Self {
            device_id,
            program_number,
            name,
            frequencies,
        })
    }

    #[allow(unused)]
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut calc = ChecksumCalculator::new(0xff);
        let mut bytes = Vec::new();
        bytes.push(SYSEX);
        bytes.push(calc.update(UNIVERSAL_NON_REAL_TIME));
        bytes.push(calc.update(self.device_id));
        bytes.push(calc.update(MIDI_TUNING));
        bytes.push(calc.update(BULK_DUMP_REPLY));
        bytes.push(calc.update(self.program_number));

        let name_bytes = self.name.as_bytes();
        let name_bytes_len = name_bytes.len();
        if name_bytes_len > 16 {
            bail!("Invalid name");
        }

        for b in name_bytes {
            bytes.push(calc.update(*b));
        }

        for _ in 0..(16 - name_bytes_len) {
            bytes.push(calc.update(0x00));
        }

        for f in &self.frequencies {
            bytes.push(calc.update(f.xx()));
            bytes.push(calc.update(f.yy()));
            bytes.push(calc.update(f.zz()));
        }

        bytes.push(calc.finalize(Some(BULK_DUMP_REPLY_MESSAGE_SIZE))?);
        bytes.push(EOX);

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use crate::midi::bulk_tuning_dump_reply::BulkTuningDumpReply;
    use crate::resources::RESOURCE_DIR;
    use anyhow::{anyhow, Result};
    use std::io::Read;

    #[test]
    fn basics() -> Result<()> {
        let bytes = RESOURCE_DIR
            .get_file("syx/carlos_super.syx")
            .ok_or_else(|| anyhow!("Could not load tuning dump"))?
            .contents();
        assert_eq!(408, bytes.len());
        let reply = BulkTuningDumpReply::from_bytes(bytes.bytes())?;
        let output = reply.to_bytes()?;
        assert_eq!(408, output.len());
        assert_eq!(bytes, output);
        Ok(())
    }
}
