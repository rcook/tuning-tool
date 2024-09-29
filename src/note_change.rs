use crate::consts::{MIDI_TUNING, NOTE_CHANGE, UNIVERSAL_REAL_TIME};
use crate::note_change_entry::NoteChangeEntry;
use anyhow::{bail, Result};
use midly::num::u7;

#[derive(Debug)]
pub(crate) struct NoteChange {
    device_id: u7,
    preset: u7,
    entries: Vec<NoteChangeEntry>,
}

impl NoteChange {
    pub(crate) fn new(device_id: u7, preset: u7, entries: &[NoteChangeEntry]) -> Result<Self> {
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
    pub(crate) const fn device_id(&self) -> u7 {
        self.device_id
    }

    #[allow(unused)]
    pub(crate) const fn preset(&self) -> u7 {
        self.preset
    }

    #[allow(unused)]
    pub(crate) fn entries(&self) -> &Vec<NoteChangeEntry> {
        &self.entries
    }
}

impl NoteChange {
    pub(crate) fn to_vec(&self) -> Result<Vec<u7>> {
        let entry_count = self.entries.len();
        let message_len = 6 + entry_count * 4;
        let entry_count: u8 = entry_count.try_into()?;
        #[allow(clippy::unnecessary_fallible_conversions)]
        let entry_count: u7 = entry_count.try_into()?;

        let mut values = Vec::with_capacity(message_len);
        values.push(UNIVERSAL_REAL_TIME);
        values.push(self.device_id);
        values.push(MIDI_TUNING);
        values.push(NOTE_CHANGE);
        values.push(self.preset);
        values.push(entry_count);

        for e in &self.entries {
            values.push(e.kk);
            values.push(e.mts.note_number.0);
            values.push(e.mts.yy);
            values.push(e.mts.zz);
        }

        assert_eq!(message_len, values.len());

        Ok(values)
    }
}
