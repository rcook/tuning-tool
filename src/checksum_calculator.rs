use crate::consts::U7_MAX;
use anyhow::{bail, Result};
use midly::num::u7;

pub(crate) struct ChecksumCalculator {
    count: usize,
    value: u7,
}

impl ChecksumCalculator {
    pub(crate) const fn new() -> Self {
        Self {
            count: 0,
            value: U7_MAX,
        }
    }

    pub(crate) fn update(&mut self, value: u7) -> u7 {
        self.count += 1;
        self.value = u7::from_int_lossy(self.value.as_int() ^ value.as_int());
        value
    }

    pub(crate) fn update_from_slice<'a>(&mut self, values: &'a [u7]) -> &'a [u7] {
        for value in values {
            _ = self.update(*value);
        }
        values
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
            assert_eq!(expected_count, self.count);
            if expected_count != self.count {
                bail!(
                    "Checksum item count {count} was not expected value {expected_count}",
                    count = self.count
                )
            }
        }
        Ok(self.value)
    }
}
