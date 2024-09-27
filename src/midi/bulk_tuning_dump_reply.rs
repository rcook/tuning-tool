use crate::midi::checksum_calculator::ChecksumCalculator;
use crate::midi::consts::{
    BULK_DUMP_REPLY, BULK_DUMP_REPLY_CHECKSUM_COUNT, BULK_DUMP_REPLY_MESSAGE_SIZE, EOX,
    MIDI_TUNING, SYSEX, UNIVERSAL_NON_REAL_TIME,
};
use crate::midi::midi_frequency::MidiFrequency;
use crate::u7::U7;
use anyhow::{bail, Result};
use std::io::{Bytes, Read};

macro_rules! read_u7 {
    ($iter: expr) => {{
        std::convert::TryInto::<crate::u7::U7>::try_into(read_u8!($iter))?
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
            .ok_or_else(|| ::anyhow::anyhow!("Failed to read byte"))?;
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
    device_id: U7,

    #[allow(unused)]
    preset: U7,

    #[allow(unused)]
    name: String,

    #[allow(unused)]
    frequencies: [MidiFrequency; 128],
}

impl BulkTuningDumpReply {
    pub(crate) fn new(
        device_id: U7,
        preset: U7,
        name: &str,
        frequencies: [MidiFrequency; 128],
    ) -> Result<Self> {
        if name.len() > 16 {
            bail!("Invalid name");
        }

        Ok(Self {
            device_id,
            preset,
            name: String::from(name),
            frequencies,
        })
    }

    #[allow(unused)]
    #[must_use]
    pub(crate) const fn device_id(&self) -> U7 {
        self.device_id
    }

    #[allow(unused)]
    #[must_use]
    pub(crate) const fn preset(&self) -> U7 {
        self.preset
    }

    #[allow(unused)]
    #[must_use]
    pub(crate) fn name(&self) -> &str {
        self.name.as_str()
    }

    #[allow(unused)]
    #[must_use]
    pub(crate) fn frequencies(&self) -> &[MidiFrequency; 128] {
        &self.frequencies
    }
}

impl BulkTuningDumpReply {
    // https://midi.org/midi-tuning-updated-specification
    pub(crate) fn from_bytes<R: Read>(bytes: Bytes<R>) -> Result<Self> {
        let mut calc = ChecksumCalculator::new();

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

        let preset = calc.update(read_u7!(iter));

        let name_values = read_u7!(iter, 16);

        for value in &name_values {
            _ = calc.update(*value)
        }

        let name = String::from(U7::to_utf8_lossy(&name_values).trim());

        let frequencies: [MidiFrequency; 128] = (0..128)
            .map(|_| {
                let xx = read_u7!(iter);
                let yy = read_u7!(iter);
                let zz = read_u7!(iter);
                Ok(MidiFrequency::new(xx, yy, zz))
            })
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .expect("Vector must have exactly 128 elements");

        for f in &frequencies {
            _ = calc.update(f.note_number());
            _ = calc.update(f.yy());
            _ = calc.update(f.zz());
        }

        let checksum = read_u7!(iter);

        if read_u8!(iter) != EOX {
            bail!("EOX not found");
        }

        calc.verify(checksum, Some(BULK_DUMP_REPLY_CHECKSUM_COUNT))?;

        Ok(Self {
            device_id,
            preset,
            name,
            frequencies,
        })
    }

    #[allow(unused)]
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut calc = ChecksumCalculator::new();
        let mut bytes = Vec::with_capacity(BULK_DUMP_REPLY_MESSAGE_SIZE);
        bytes.push(SYSEX);
        bytes.push(calc.update(UNIVERSAL_NON_REAL_TIME).as_u8());
        bytes.push(calc.update(self.device_id).as_u8());
        bytes.push(calc.update(MIDI_TUNING).as_u8());
        bytes.push(calc.update(BULK_DUMP_REPLY).as_u8());
        bytes.push(calc.update(self.preset).as_u8());

        let name_bytes = self.name.as_bytes();
        let name_bytes_len = name_bytes.len();
        if name_bytes_len > 16 {
            bail!("Invalid name");
        }

        for b in name_bytes {
            bytes.push(calc.update((*b).try_into()?).as_u8());
        }

        for _ in 0..(16 - name_bytes_len) {
            bytes.push(calc.update(U7::ZERO).as_u8());
        }

        for f in &self.frequencies {
            bytes.push(calc.update(f.note_number()).as_u8());
            bytes.push(calc.update(f.yy()).as_u8());
            bytes.push(calc.update(f.zz()).as_u8());
        }

        bytes.push(calc.finalize(Some(BULK_DUMP_REPLY_CHECKSUM_COUNT))?.as_u8());
        bytes.push(EOX);

        assert_eq!(BULK_DUMP_REPLY_MESSAGE_SIZE, bytes.len());

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use crate::midi::bulk_tuning_dump_reply::BulkTuningDumpReply;
    use crate::midi::consts::BULK_DUMP_REPLY_MESSAGE_SIZE;
    use crate::resources::RESOURCE_DIR;
    use anyhow::{anyhow, Result};
    use std::io::Read;

    #[test]
    fn basics() -> Result<()> {
        let bytes = RESOURCE_DIR
            .get_file("syx/carlos_super.syx")
            .ok_or_else(|| anyhow!("Could not load tuning dump"))?
            .contents();
        assert_eq!(BULK_DUMP_REPLY_MESSAGE_SIZE, bytes.len());
        let reply = BulkTuningDumpReply::from_bytes(bytes.bytes())?;
        let output = reply.to_bytes()?;
        assert_eq!(BULK_DUMP_REPLY_MESSAGE_SIZE, output.len());
        assert_eq!(bytes, output);
        Ok(())
    }
}
