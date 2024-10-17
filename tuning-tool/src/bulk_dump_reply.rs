// Copyright (c) 2024 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

use crate::checksum_calculator::ChecksumCalculator;
use crate::consts::{
    BULK_DUMP_REPLY, BULK_DUMP_REPLY_CHECKSUM_COUNT, EOX, MIDI_TUNING, SYSEX,
    UNIVERSAL_NON_REAL_TIME,
};
use crate::mts_entry::MtsEntry;
use crate::note_number::NoteNumber;
use crate::preset_name::PresetName;
use crate::read::{read, read_multi};
use crate::types::{Char7, Checksum, DeviceId, Lsb, MidiValue, Msb, Preset};
use anyhow::{bail, Result};
use std::io::{Bytes, Read};

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
    device_id: DeviceId,
    preset: Preset,
    name: PresetName,
    entries: MtsEntries,
}

impl BulkDumpReply {
    #[cfg(test)]
    pub(crate) fn new(
        device_id: DeviceId,
        preset: Preset,
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

    pub(crate) const fn device_id(&self) -> DeviceId {
        self.device_id
    }

    pub(crate) const fn preset(&self) -> Preset {
        self.preset
    }

    pub(crate) fn name(&self) -> &PresetName {
        &self.name
    }

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

        if calc.update(read::<MidiValue, _>(&mut iter)?) != UNIVERSAL_NON_REAL_TIME {
            bail!("Unsupported header");
        }

        let device_id = calc.update(read::<DeviceId, _>(&mut iter)?);

        if calc.update(read::<MidiValue, _>(&mut iter)?) != MIDI_TUNING {
            bail!("Expected MIDI Tuning")
        }

        if calc.update(read::<MidiValue, _>(&mut iter)?) != BULK_DUMP_REPLY {
            bail!("Expected Bulk Dump reply")
        }

        let preset = calc.update(read::<Preset, _>(&mut iter)?);

        let name = PresetName::new(read_multi::<Char7, _, { PresetName::LEN }>(&mut iter)?);
        _ = calc.update_from_slice(name.as_array());

        let entries: MtsEntries = (0..ENTRIES_LEN)
            .map(|_| {
                let note_number = read::<NoteNumber, _>(&mut iter)?;
                let msb = read::<Msb, _>(&mut iter)?;
                let lsb = read::<Lsb, _>(&mut iter)?;
                Ok(MtsEntry {
                    note_number,
                    msb,
                    lsb,
                })
            })
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .expect("Vector must have exactly 128 elements");

        for e in &entries {
            _ = calc.update(e.note_number);
            _ = calc.update(e.msb);
            _ = calc.update(e.lsb);
        }

        let checksum = read::<Checksum, _>(&mut iter)?;

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

    #[cfg(test)]
    pub(crate) fn to_vec(&self) -> Result<Vec<MidiValue>> {
        use crate::consts::BULK_DUMP_REPLY_MESSAGE_SIZE;
        use crate::midi_message_builder::MidiMessageBuilder;

        let mut calc = ChecksumCalculator::new();
        let mut values = MidiMessageBuilder::with_required_len(BULK_DUMP_REPLY_MESSAGE_SIZE);
        values.push(calc.update(UNIVERSAL_NON_REAL_TIME));
        values.push(calc.update(self.device_id));
        values.push(calc.update(MIDI_TUNING));
        values.push(calc.update(BULK_DUMP_REPLY));
        values.push(calc.update(self.preset));

        values.extend_from_slice(calc.update_from_slice(self.name.as_array()));

        for e in &self.entries {
            values.push(calc.update(e.note_number));
            values.push(calc.update(e.msb));
            values.push(calc.update(e.lsb));
        }

        values.push(calc.finalize(Some(BULK_DUMP_REPLY_CHECKSUM_COUNT))?);

        values.finalize()
    }

    #[cfg(test)]
    pub(crate) fn to_bytes_with_start_and_end(&self) -> Result<Vec<u8>> {
        use crate::consts::BULK_DUMP_REPLY_MESSAGE_SIZE;

        let vec = self.to_vec()?;
        let inner_bytes = MidiValue::to_u8_slice(&vec);
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
    use crate::evaluate::Evaluate;
    use crate::resources::include_resource_bytes;
    use crate::types::Preset;
    use anyhow::Result;
    use std::io::Read;

    macro_rules! verify_bytes {
        ($path: expr, $preset: expr, $name: expr, $reference_key: expr, $reference_frequency: expr) => {{
            use crate::bulk_dump_reply::BulkDumpReply;
            use crate::bulk_dump_reply::MtsEntries;
            use crate::frequency::Frequency;
            use crate::key_frequency_mapping::compute_symbolic;
            use crate::key_mappings::KeyMappings;
            use crate::keyboard_mapping::KeyboardMapping;
            use crate::resources::{include_resource_bytes, include_resource_str};
            use crate::scl_file::SclFile;
            use crate::types::{DeviceId, KeyNumber};
            use std::assert_eq;

            let scala_file = include_resource_str!("carlos_super.scl")
                .parse::<SclFile>()
                .expect("Must succeed");
            let scale = scala_file.scale();

            let keyboard_mapping = KeyboardMapping::new(
                KeyNumber::ZERO,
                KeyNumber::MAX,
                $reference_key,
                $reference_key,
                $reference_frequency,
                KeyMappings::Linear,
            )
            .expect("Must succeed");

            let entries: MtsEntries = compute_symbolic(scale, &keyboard_mapping)
                .expect("Must succeed")
                .iter()
                .map(|mapping| {
                    Frequency(mapping.frequency.as_f64())
                        .to_mts_entry()
                        .expect("Must succeed")
                })
                .collect::<Vec<_>>()
                .try_into()
                .expect("Must have exactly 128 elements");
            let reply = BulkDumpReply::new(
                DeviceId::ZERO,
                $preset,
                $name.parse().expect("Must succeed"),
                entries,
            )
            .expect("Must succeed");

            let ref_bytes = include_resource_bytes!($path).to_vec();
            let bytes = reply.to_bytes_with_start_and_end().expect("Must succeed");
            assert_eq!(ref_bytes, bytes);
        }};
    }

    #[test]
    fn basics() -> Result<()> {
        let bytes = include_resource_bytes!("carlos_super.syx").to_vec();
        assert_eq!(BULK_DUMP_REPLY_MESSAGE_SIZE + 2, bytes.len());
        let reply = BulkDumpReply::from_bytes(bytes.bytes())?;
        let output = reply.to_bytes_with_start_and_end()?;
        assert_eq!(BULK_DUMP_REPLY_MESSAGE_SIZE + 2, output.len());
        assert_eq!(bytes, output);
        Ok(())
    }

    #[test]
    fn reference_key_0_min() {
        verify_bytes!(
            "carlos_super.syx",
            Preset::constant::<8>(),
            "carlos_super.mid",
            KeyNumber::ZERO,
            Frequency::MIN
        );
    }

    #[test]
    fn reference_key_69_concert_a() {
        verify_bytes!(
            "carlos_super_a4.syx",
            Preset::ZERO,
            "carlos_super_a4 ",
            KeyNumber::constant::<69>(),
            Frequency::CONCERT_A4
        );
    }
}
