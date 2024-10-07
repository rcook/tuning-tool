use crate::frequency::Frequency;
use crate::key_mappings::KeyMappings;
use crate::types::KeyNumber;
use anyhow::{bail, Result};

#[derive(Debug)]
pub(crate) struct KeyboardMapping {
    start_key: KeyNumber,
    end_key: KeyNumber,
    reference_key: KeyNumber,
    reference_frequency: Frequency,
    key_mappings: KeyMappings,
}

impl KeyboardMapping {
    pub(crate) fn new(
        start_key: KeyNumber,
        end_key: KeyNumber,
        reference_key: KeyNumber,
        reference_frequency: Frequency,
        key_mappings: KeyMappings,
    ) -> Result<Self> {
        if end_key.checked_sub(start_key).is_none() {
            bail!("Invalid end key");
        }

        Ok(Self {
            start_key,
            end_key,
            reference_key,
            reference_frequency,
            key_mappings,
        })
    }

    #[cfg(test)]
    pub(crate) fn new_full(
        reference_key: KeyNumber,
        reference_frequency: Frequency,
        key_mappings: KeyMappings,
    ) -> Result<Self> {
        Self::new(
            KeyNumber::ZERO,
            KeyNumber::MAX,
            reference_key,
            reference_frequency,
            key_mappings,
        )
    }

    #[cfg(test)]
    pub(crate) fn new_full_linear(
        reference_key: KeyNumber,
        reference_frequency: Frequency,
    ) -> Result<Self> {
        Self::new_full(reference_key, reference_frequency, KeyMappings::Linear)
    }

    pub(crate) const fn start_key(&self) -> &KeyNumber {
        &self.start_key
    }

    pub(crate) const fn end_key(&self) -> &KeyNumber {
        &self.end_key
    }

    pub(crate) const fn reference_key(&self) -> &KeyNumber {
        &self.reference_key
    }

    pub(crate) const fn reference_frequency(&self) -> &Frequency {
        &self.reference_frequency
    }

    pub(crate) const fn key_mappings(&self) -> &KeyMappings {
        &self.key_mappings
    }
}
