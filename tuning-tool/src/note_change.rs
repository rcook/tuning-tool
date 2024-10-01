use crate::consts::{MIDI_TUNING, NOTE_CHANGE, UNIVERSAL_REAL_TIME};
use crate::device_id::DeviceId;
use crate::midi_message_builder::MidiMessageBuilder;
use crate::midi_value::MidiValue;
use crate::note_change_entry::NoteChangeEntry;
use crate::preset::Preset;
use anyhow::{bail, Result};
use midly::num::u7;

#[derive(Debug)]
pub(crate) struct NoteChange {
    device_id: DeviceId,
    preset: Preset,
    entries: Vec<NoteChangeEntry>,
}

impl NoteChange {
    #[allow(unused)]
    pub(crate) fn new(
        device_id: DeviceId,
        preset: Preset,
        entries: &[NoteChangeEntry],
    ) -> Result<Self> {
        if entries.len() > 127 {
            bail!("Too many note changes")
        }
        Ok(Self {
            device_id,
            preset,
            entries: entries.to_vec(),
        })
    }

    #[allow(unused)]
    pub(crate) const fn device_id(&self) -> DeviceId {
        self.device_id
    }

    #[allow(unused)]
    pub(crate) const fn preset(&self) -> Preset {
        self.preset
    }

    #[allow(unused)]
    pub(crate) fn entries(&self) -> &Vec<NoteChangeEntry> {
        &self.entries
    }
}

impl NoteChange {
    #[allow(unused)]
    pub(crate) fn to_vec(&self) -> Result<Vec<MidiValue>> {
        let entry_count = self.entries.len();
        let message_len = 6 + entry_count * 4;
        let entry_count: u8 = entry_count.try_into()?;
        #[allow(clippy::unnecessary_fallible_conversions)]
        let entry_count: u7 = entry_count.try_into()?;

        let mut values = MidiMessageBuilder::with_required_len(message_len);
        values.push(UNIVERSAL_REAL_TIME);
        values.push(self.device_id.to_u7());
        values.push(MIDI_TUNING);
        values.push(NOTE_CHANGE);
        values.push(self.preset.to_u7());
        values.push(entry_count);

        for e in &self.entries {
            values.push(e.kk);
            values.push(e.mts.note_number.to_u7());
            values.push(e.mts.msb.to_u7());
            values.push(e.mts.lsb.to_u7());
        }

        values.finalize()
    }
}

#[cfg(test)]
mod tests {
    use crate::frequencies::calculate_frequencies;
    use crate::frequency::Frequency;
    use crate::hex_dump::from_hex_dump;
    use crate::keyboard_mapping::KeyboardMapping;
    use crate::note_change::NoteChange;
    use crate::note_change_entry::NoteChangeEntry;
    use crate::note_number::NoteNumber;
    use crate::preset::Preset;
    use crate::resources::RESOURCE_DIR;
    use crate::scl_file::SclFile;
    use crate::{coerce::unsafe_coerce_slice_to_u7_slice, device_id::DeviceId};
    use anyhow::{anyhow, Result};
    use midly::live::{LiveEvent, SystemCommon};
    use std::iter::zip;

    #[test]
    fn basics() -> Result<()> {
        const EXPECTED_MESSAGE_HEX_DUMPS: [&str; 2] = [
            "F0 7F 00 08 02 08 40 00 00 00 00 01 01 06 2C 02 02 05 01 03 03 14 03 04 03 6E 3E 05 04 7D 40 06
            05 41 58 07 07 02 40 08 08 33 70 09 08 6B 7D 0A 09 58 0C 0B 0A 70 7E 0C 0C 00 00 0D 0D 06 2C 0E
            0E 05 01 0F 0F 14 03 10 0F 6E 3E 11 10 7D 40 12 11 41 58 13 13 02 40 14 14 33 70 15 14 6B 7D 16
            15 58 0C 17 16 70 7E 18 18 00 00 19 19 06 2C 1A 1A 05 01 1B 1B 14 03 1C 1B 6E 3E 1D 1C 7D 40 1E
            1D 41 58 1F 1F 02 40 20 20 33 70 21 20 6B 7D 22 21 58 0C 23 22 70 7E 24 24 00 00 25 25 06 2C 26
            26 05 01 27 27 14 03 28 27 6E 3E 29 28 7D 40 2A 29 41 58 2B 2B 02 40 2C 2C 33 70 2D 2C 6B 7D 2E
            2D 58 0C 2F 2E 70 7E 30 30 00 00 31 31 06 2C 32 32 05 01 33 33 14 03 34 33 6E 3E 35 34 7D 40 36
            35 41 58 37 37 02 40 38 38 33 70 39 38 6B 7D 3A 39 58 0C 3B 3A 70 7E 3C 3C 00 00 3D 3D 06 2C 3E
            3E 05 01 3F 3F 14 03 F7",
            "F0 7F 00 08 02 08 40 40 3F 6E 3E 41 40 7D 40 42 41 41 58 43 43 02 40 44 44 33 70 45 44 6B 7D 46
            45 58 0C 47 46 70 7E 48 48 00 00 49 49 06 2C 4A 4A 05 01 4B 4B 14 03 4C 4B 6E 3E 4D 4C 7D 40 4E
            4D 41 58 4F 4F 02 40 50 50 33 70 51 50 6B 7D 52 51 58 0C 53 52 70 7E 54 54 00 00 55 55 06 2C 56
            56 05 01 57 57 14 03 58 57 6E 3E 59 58 7D 40 5A 59 41 58 5B 5B 02 40 5C 5C 33 70 5D 5C 6B 7D 5E
            5D 58 0C 5F 5E 70 7E 60 60 00 00 61 61 06 2C 62 62 05 01 63 63 14 03 64 63 6E 3E 65 64 7D 40 66
            65 41 58 67 67 02 40 68 68 33 70 69 68 6B 7D 6A 69 58 0C 6B 6A 70 7E 6C 6C 00 00 6D 6D 06 2C 6E
            6E 05 01 6F 6F 14 03 70 6F 6E 3E 71 70 7D 40 72 71 41 58 73 73 02 40 74 74 33 70 75 74 6B 7D 76
            75 58 0C 77 76 70 7E 78 78 00 00 79 79 06 2C 7A 7A 05 01 7B 7B 14 03 7C 7B 6E 3E 7D 7C 7D 40 7E
            7D 41 58 7F 7F 02 40 F7"
        ];

        fn is_expected_bytes(expected_hex_dump: &str, bytes: &[u8]) -> Result<bool> {
            let expected_bytes = from_hex_dump(expected_hex_dump)?;
            Ok(bytes == expected_bytes)
        }

        let scala_file = RESOURCE_DIR
            .get_file("scl/carlos_super.scl")
            .ok_or_else(|| anyhow!("Could not get scl file"))?
            .contents_utf8()
            .ok_or_else(|| anyhow!("Could not convert to string"))?
            .parse::<SclFile>()?;

        let keyboard_mapping = KeyboardMapping::new(
            NoteNumber::ZERO,
            NoteNumber::MAX,
            NoteNumber::ZERO,
            Frequency::MIDI_MIN,
        )?;

        let entries = calculate_frequencies(scala_file.scale(), &keyboard_mapping)
            .iter()
            .enumerate()
            .map(|(i, f)| {
                Ok(NoteChangeEntry {
                    #[allow(clippy::unnecessary_fallible_conversions)]
                    kk: TryInto::<u8>::try_into(i)?.try_into()?,
                    mts: f.to_mts_entry()?,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let messages = entries
            .chunks(64)
            .map(|chunk| {
                let message = NoteChange::new(DeviceId::ZERO, Preset::constant::<8>(), chunk)?;
                let values = message.to_vec()?;
                let event = LiveEvent::Common(SystemCommon::SysEx(
                    unsafe_coerce_slice_to_u7_slice(&values),
                ));
                let mut buffer = Vec::new();
                event.write_std(&mut buffer)?;
                Ok(buffer)
            })
            .collect::<Result<Vec<_>>>()?;

        assert_eq!(EXPECTED_MESSAGE_HEX_DUMPS.len(), messages.len());

        for (expected, actual) in zip(EXPECTED_MESSAGE_HEX_DUMPS, messages) {
            assert!(is_expected_bytes(expected, &actual)?);
        }
        Ok(())
    }
}
