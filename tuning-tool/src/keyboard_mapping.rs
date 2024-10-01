use crate::frequency::Frequency;
use crate::note_number::NoteNumber;
use anyhow::{bail, Result};

#[derive(Debug)]
pub(crate) struct KeyboardMapping {
    start_note_number: NoteNumber,
    end_note_number: NoteNumber,
    base_note_number: NoteNumber,
    base_frequency: Frequency,
}

impl KeyboardMapping {
    pub(crate) fn new(
        start_note_number: NoteNumber,
        end_note_number: NoteNumber,
        base_note_number: NoteNumber,
        base_frequency: Frequency,
    ) -> Result<Self> {
        if end_note_number.checked_sub(start_note_number).is_none() {
            bail!("Invalid end note number");
        }

        Ok(Self {
            start_note_number,
            end_note_number,
            base_note_number,
            base_frequency,
        })
    }

    pub(crate) const fn start_note_number(&self) -> &NoteNumber {
        &self.start_note_number
    }

    pub(crate) const fn end_note_number(&self) -> &NoteNumber {
        &self.end_note_number
    }

    pub(crate) const fn base_note_number(&self) -> &NoteNumber {
        &self.base_note_number
    }

    pub(crate) const fn base_frequency(&self) -> &Frequency {
        &self.base_frequency
    }
}
