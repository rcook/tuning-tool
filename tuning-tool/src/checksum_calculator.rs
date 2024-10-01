use crate::checksum::Checksum;
use anyhow::{bail, Result};
use midly::num::u7;
use tuning_tool_lib::U7;

pub(crate) struct ChecksumCalculator {
    count: usize,
    value: Checksum,
}

impl ChecksumCalculator {
    pub(crate) const fn new() -> Self {
        Self {
            count: 0,
            value: Checksum::MAX,
        }
    }

    pub(crate) fn update<U: Copy + U7>(&mut self, value: U) -> U {
        self.count += 1;
        self.value = Checksum::from_u8_lossy(self.value.to_u8() ^ value.to_u8());
        value
    }

    pub(crate) fn update_from_slice<'a>(&mut self, values: &'a [u7]) -> &'a [u7] {
        for value in values {
            _ = self.update(*value);
        }
        values
    }

    pub(crate) fn verify(
        self,
        expected_checksum: Checksum,
        expected_count: Option<usize>,
    ) -> Result<()> {
        let checksum = self.finalize(expected_count)?;
        if checksum != expected_checksum {
            bail!("Checksum validation failed")
        }
        Ok(())
    }

    pub(crate) fn finalize(self, expected_count: Option<usize>) -> Result<Checksum> {
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
