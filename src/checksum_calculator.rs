use crate::u7::u7;
use anyhow::{bail, Result};

pub(crate) struct ChecksumCalculator {
    count: usize,
    value: u7,
}

impl ChecksumCalculator {
    pub(crate) const fn new() -> Self {
        Self {
            count: 0,
            value: u7::MAX,
        }
    }

    #[must_use]
    pub(crate) fn update(&mut self, value: u7) -> u7 {
        self.count += 1;
        self.value ^= value;
        value
    }

    pub(crate) fn verify(self, expected_checksum: u7, expected_count: Option<usize>) -> Result<()> {
        let checksum = self.finalize(expected_count)?;
        if checksum != expected_checksum {
            bail!("Checksum validation failed")
        }
        Ok(())
    }

    pub(crate) fn finalize(self, expected_count: Option<usize>) -> Result<u7> {
        if let Some(expected_count) = expected_count {
            if expected_count != self.count {
                bail!("Checksum item count was incorrect")
            }
        }
        Ok(self.value)
    }
}