use crate::frequency::Frequency;
use crate::types::KeyNumber;
use anyhow::{bail, Result};

#[derive(Debug)]
pub(crate) struct KeyboardMapping {
    start_key: KeyNumber,
    end_key: KeyNumber,
    reference_key: KeyNumber,
    reference_frequency: Frequency,
}

impl KeyboardMapping {
    pub(crate) fn new(
        start_key: KeyNumber,
        end_key: KeyNumber,
        reference_key: KeyNumber,
        reference_frequency: Frequency,
    ) -> Result<Self> {
        if end_key.checked_sub(start_key).is_none() {
            bail!("Invalid end note number");
        }

        Ok(Self {
            start_key,
            end_key,
            reference_key,
            reference_frequency,
        })
    }

    pub(crate) const fn start_key(&self) -> &KeyNumber {
        &self.start_key
    }

    pub(crate) const fn end_key(&self) -> &KeyNumber {
        &self.end_key
    }

    pub(crate) const fn base_note_number(&self) -> &KeyNumber {
        &self.reference_key
    }

    pub(crate) const fn base_frequency(&self) -> &Frequency {
        &self.reference_frequency
    }
}
