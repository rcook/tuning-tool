use crate::checksum_calculator::ChecksumCalculator;
use crate::consts::{
    BULK_DUMP_REPLY, BULK_DUMP_REPLY_CHECKSUM_COUNT, BULK_DUMP_REPLY_MESSAGE_SIZE, EOX,
    MIDI_TUNING, SYSEX, UNIVERSAL_NON_REAL_TIME,
};
use crate::mts_entry::MtsEntry;
use crate::note_number::NoteNumber;
use crate::preset_name::PresetName;
use anyhow::{bail, Result};
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

const ENTRIES_LEN: usize = 128;

pub(crate) type MtsEntries = [MtsEntry; ENTRIES_LEN];

#[derive(Debug)]
pub(crate) struct BulkDumpReply {
    device_id: u7,
    preset: u7,
    name: PresetName,
    entries: MtsEntries,
}

impl BulkDumpReply {
    pub(crate) fn new(
        device_id: u7,
        preset: u7,
        name: PresetName,
        entries: MtsEntries,
    ) -> Result<Self> {
        Ok(Self {
            device_id,
            preset,
            name,
            entries,
        })
    }

    #[allow(unused)]
    pub(crate) const fn device_id(&self) -> u7 {
        self.device_id
    }

    #[allow(unused)]
    pub(crate) const fn preset(&self) -> u7 {
        self.preset
    }

    #[allow(unused)]
    pub(crate) fn name(&self) -> &PresetName {
        &self.name
    }

    #[allow(unused)]
    pub(crate) fn entries(&self) -> &MtsEntries {
        &self.entries
    }
}

impl BulkDumpReply {
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

        let entries: MtsEntries = (0..ENTRIES_LEN)
            .map(|_| {
                let xx = read_u7!(iter);
                let yy = read_u7!(iter);
                let zz = read_u7!(iter);
                Ok(MtsEntry {
                    note_number: NoteNumber::try_from(xx.as_int())?,
                    yy,
                    zz,
                })
            })
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .expect("Vector must have exactly 128 elements");

        for e in &entries {
            _ = calc.update(u7::from_int_lossy(e.note_number.to_u8()));
            _ = calc.update(e.yy);
            _ = calc.update(e.zz);
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
            entries,
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

        for e in &self.entries {
            values.push(calc.update(u7::from_int_lossy(e.note_number.to_u8())));
            values.push(calc.update(e.yy));
            values.push(calc.update(e.zz));
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
    use crate::consts::{BULK_DUMP_REPLY_MESSAGE_SIZE, U7_ZERO};
    use crate::frequencies::calculate_frequencies;
    use crate::frequency::Frequency;
    use crate::keyboard_mapping::KeyboardMapping;
    use crate::note_number::NoteNumber;
    use crate::test_util::{read_test_scl_file, read_test_syx_file};
    use anyhow::Result;
    use midly::num::u7;
    use std::io::Read;
    use std::path::Path;

    #[test]
    fn basics() -> Result<()> {
        let bytes = read_test_syx_file("carlos_super.syx")?;
        assert_eq!(BULK_DUMP_REPLY_MESSAGE_SIZE + 2, bytes.len());
        let reply = BulkDumpReply::from_bytes(bytes.bytes())?;
        let output = reply.to_bytes_with_start_and_end()?;
        assert_eq!(BULK_DUMP_REPLY_MESSAGE_SIZE + 2, output.len());
        assert_eq!(bytes, output);
        Ok(())
    }

    #[test]
    fn base_note_number_0() -> Result<()> {
        check_bytes(
            "carlos_super.syx",
            u7::from_int_lossy(8),
            "carlos_super.mid",
            NoteNumber::ZERO,
            Frequency::MIDI_MIN,
        )?;
        Ok(())
    }

    #[test]
    fn base_note_number_69() -> Result<()> {
        check_bytes(
            "carlos_super_a4.syx",
            u7::from_int_lossy(0),
            "carlos_super_a4 ",
            NoteNumber::A4,
            NoteNumber::A4.to_frequency(),
        )?;
        Ok(())
    }

    fn check_bytes<P: AsRef<Path>>(
        expected_syx_path: P,
        preset: u7,
        name: &str,
        base_note_number: NoteNumber,
        base_frequency: Frequency,
    ) -> Result<()> {
        let scala_file = read_test_scl_file("scl/carlos_super.scl")?;
        let scale = scala_file.scale();

        let keyboard_mapping = KeyboardMapping::new(
            NoteNumber::ZERO,
            NoteNumber::MAX,
            base_note_number,
            base_frequency,
        )?;

        let entries = calculate_frequencies(scale, &keyboard_mapping)
            .iter()
            .map(|f| f.to_mts_entry())
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .expect("Must have exactly 128 elements");
        let reply = BulkDumpReply::new(U7_ZERO, preset, name.parse()?, entries)?;

        let ref_bytes = read_test_syx_file(expected_syx_path)?;
        let bytes = reply.to_bytes_with_start_and_end()?;
        assert_eq!(ref_bytes, bytes);
        Ok(())
    }
}
