use crate::u7::U7;
use anyhow::{bail, Result};

pub(crate) struct ChecksumCalculator {
    count: usize,
    value: U7,
}

impl ChecksumCalculator {
    pub(crate) const fn new() -> Self {
        Self {
            count: 0,
            value: U7::MAX,
        }
    }

    #[must_use]
    pub(crate) fn update(&mut self, value: U7) -> U7 {
        self.count += 1;
        self.value ^= value;
        value
    }

    pub(crate) fn verify(self, expected_checksum: U7, expected_count: Option<usize>) -> Result<()> {
        let checksum = self.finalize(expected_count)?;
        if checksum != expected_checksum {
            bail!("Checksum validation failed")
        }
        Ok(())
    }

    pub(crate) fn finalize(self, expected_count: Option<usize>) -> Result<U7> {
        if let Some(expected_count) = expected_count {
            if expected_count != self.count {
                bail!("Checksum item count was incorrect")
            }
        }
        Ok(self.value)
    }
}
