use crate::frequency::Frequency;
use crate::note_number::NoteNumber;
use anyhow::{bail, Result};

#[derive(Debug)]
pub(crate) struct KeyboardMapping {
    start_key: NoteNumber,
    end_key: NoteNumber,
    base_note_number: NoteNumber,
    base_frequency: Frequency,
}

impl KeyboardMapping {
    pub(crate) fn new(
        start_key: NoteNumber,
        end_key: NoteNumber,
        base_note_number: NoteNumber,
        base_frequency: Frequency,
    ) -> Result<Self> {
        if end_key.checked_sub(start_key).is_none() {
            bail!("Invalid end note number");
        }

        Ok(Self {
            start_key,
            end_key,
            base_note_number,
            base_frequency,
        })
    }

    pub(crate) const fn start_key(&self) -> &NoteNumber {
        &self.start_key
    }

    pub(crate) const fn end_key(&self) -> &NoteNumber {
        &self.end_key
    }

    pub(crate) const fn base_note_number(&self) -> &NoteNumber {
        &self.base_note_number
    }

    pub(crate) const fn base_frequency(&self) -> &Frequency {
        &self.base_frequency
    }
}
