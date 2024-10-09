// Copyright (c) 2024 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

use crate::types::Checksum;
use anyhow::{bail, Result};
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

    pub(crate) fn update_from_slice<'a, U: Copy + U7>(&mut self, values: &'a [U]) -> &'a [U] {
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
