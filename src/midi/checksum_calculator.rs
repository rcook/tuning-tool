use anyhow::{bail, Result};

pub(crate) struct ChecksumCalculator {
    count: usize,
    value: u8,
}

impl ChecksumCalculator {
    pub(crate) fn new(value: u8) -> Self {
        Self { count: 0, value }
    }

    #[must_use]
    pub(crate) fn update(&mut self, value: u8) -> u8 {
        self.count += 1;
        self.value ^= value;
        value
    }

    pub(crate) fn verify(self, expected_checksum: u8, expected_count: Option<usize>) -> Result<()> {
        let checksum = self.finalize(expected_count)?;
        if checksum != expected_checksum {
            bail!("Checksum validation failed")
        }
        Ok(())
    }

    pub(crate) fn finalize(self, expected_count: Option<usize>) -> Result<u8> {
        if let Some(expected_count) = expected_count {
            if expected_count != self.count {
                bail!("Checksum item count was incorrect")
            }
        }
        Ok(self.value & 0x7f)
    }
}
