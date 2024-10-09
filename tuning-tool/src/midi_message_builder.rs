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

use crate::types::MidiValue;
use anyhow::{bail, Result};
use tuning_tool_lib::U7;

pub(crate) struct MidiMessageBuilder {
    required_len: usize,
    values: Vec<MidiValue>,
}

impl MidiMessageBuilder {
    pub(crate) fn with_required_len(required_len: usize) -> Self {
        Self {
            required_len,
            values: Vec::with_capacity(required_len),
        }
    }

    pub(crate) fn push<U: U7>(&mut self, value: U) -> U {
        self.values.push(MidiValue::from_u8_lossy(value.to_u8()));
        value
    }

    #[cfg(test)]
    pub(crate) fn extend_from_slice<U: U7>(&mut self, other: &[U]) {
        self.values
            .extend(other.iter().map(|x| MidiValue::from_u8_lossy(x.to_u8())))
    }

    pub(crate) fn finalize(self) -> Result<Vec<MidiValue>> {
        if self.values.len() != self.required_len {
            bail!("MIDI value vector was not expected length")
        }
        Ok(self.values)
    }
}
