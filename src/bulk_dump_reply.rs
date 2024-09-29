use crate::checksum_calculator::ChecksumCalculator;
use crate::consts::{
    BULK_DUMP_REPLY, BULK_DUMP_REPLY_CHECKSUM_COUNT, BULK_DUMP_REPLY_MESSAGE_SIZE, EOX,
    MIDI_TUNING, SYSEX, U7_ZERO, UNIVERSAL_NON_REAL_TIME,
};
use crate::mts_bytes::MtsBytes;
use crate::note_number::NoteNumber;
use crate::preset_name::PresetName;
use crate::string_extras::StringExtras;
use anyhow::{anyhow, bail, Result};
use midly::num::u7;
use std::io::{Bytes, Read};

macro_rules! read_u7 {
    ($iter: expr) => {
        std::convert::TryInto::<midly::num::u7>::try_into(read_u8!($iter))?
    };
    ($iter: expr, $count: expr) => {{
        let mut result = [crate::consts::U7_ZERO; $count];
        for i in 0..$count {
            result[i] = read_u7!($iter);
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
pub(crate) struct BulkDumpReply {
    device_id: u7,
    preset: u7,
    name: PresetName,
    frequencies: [MtsBytes; 128],
}

impl BulkDumpReply {
    pub(crate) fn new(
        device_id: u7,
        preset: u7,
        name: PresetName,
        frequencies: [MtsBytes; 128],
    ) -> Result<Self> {
        Ok(Self {
            device_id,
            preset,
            name,
            frequencies,
        })
    }

    pub(crate) const fn device_id(&self) -> u7 {
        self.device_id
    }

    pub(crate) const fn preset(&self) -> u7 {
        self.preset
    }

    pub(crate) fn name(&self) -> &PresetName {
        &self.name
    }

    pub(crate) fn frequencies(&self) -> &[MtsBytes; 128] {
        &self.frequencies
    }
}

impl BulkDumpReply {
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

        let name = PresetName::new(read_u7!(iter, PresetName::LEN));
        _ = calc.update_from_slice(name.as_array());

        let frequencies: [MtsBytes; 128] = (0..128)
            .map(|_| {
                let xx = read_u7!(iter);
                let yy = read_u7!(iter);
                let zz = read_u7!(iter);
                Ok(MtsBytes {
                    note_number: NoteNumber(xx.as_int() as i32),
                    yy,
                    zz,
                })
            })
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .expect("Vector must have exactly 128 elements");

        for f in &frequencies {
            _ = calc.update((f.note_number.0 as u8).into());
            _ = calc.update(f.yy);
            _ = calc.update(f.zz);
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

    pub(crate) fn to_vec(&self) -> Result<Vec<u7>> {
        let mut calc = ChecksumCalculator::new();
        let mut values = Vec::with_capacity(BULK_DUMP_REPLY_MESSAGE_SIZE + 2);
        values.push(calc.update(UNIVERSAL_NON_REAL_TIME));
        values.push(calc.update(self.device_id));
        values.push(calc.update(MIDI_TUNING));
        values.push(calc.update(BULK_DUMP_REPLY));
        values.push(calc.update(self.preset));

        values.extend_from_slice(calc.update_from_slice(self.name.as_array()));

        for f in &self.frequencies {
            values.push(calc.update((f.note_number.0 as u8).into()));
            values.push(calc.update(f.yy));
            values.push(calc.update(f.zz));
        }

        values.push(calc.finalize(Some(BULK_DUMP_REPLY_CHECKSUM_COUNT))?);

        assert_eq!(BULK_DUMP_REPLY_MESSAGE_SIZE, values.len());

        Ok(values)
    }

    pub(crate) fn to_bytes_with_start_and_end(&self) -> Result<Vec<u8>> {
        let vec = self.to_vec()?;
        let inner_bytes = u7::slice_as_int(&vec);
        let mut bytes = Vec::with_capacity(inner_bytes.len() + 2);
        bytes.push(SYSEX);
        bytes.extend_from_slice(inner_bytes);
        bytes.push(EOX);
        assert_eq!(BULK_DUMP_REPLY_MESSAGE_SIZE + 2, bytes.len());
        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use crate::bulk_dump_reply::BulkDumpReply;
    use crate::consts::BULK_DUMP_REPLY_MESSAGE_SIZE;
    use crate::resources::RESOURCE_DIR;
    use anyhow::{anyhow, Result};
    use std::io::Read;

    #[test]
    fn basics() -> Result<()> {
        let bytes = RESOURCE_DIR
            .get_file("syx/carlos_super.syx")
            .ok_or_else(|| anyhow!("Could not load tuning dump"))?
            .contents();
        assert_eq!(BULK_DUMP_REPLY_MESSAGE_SIZE + 2, bytes.len());
        let reply = BulkDumpReply::from_bytes(bytes.bytes())?;
        let output = reply.to_bytes_with_start_and_end()?;
        assert_eq!(BULK_DUMP_REPLY_MESSAGE_SIZE + 2, output.len());
        assert_eq!(bytes, output);
        Ok(())
    }
}
