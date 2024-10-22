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

use crate::{frequency::Frequency, types::KeyNumber};

#[derive(Clone, Debug)]
pub(crate) struct Reference {
    zero_key: KeyNumber,
    reference_key: KeyNumber,
    reference_frequency: Frequency,
}

impl Reference {
    pub(crate) fn new(
        zero_key: KeyNumber,
        reference_key: KeyNumber,
        reference_frequency: Frequency,
    ) -> Self {
        Self {
            zero_key,
            reference_key,
            reference_frequency,
        }
    }

    pub(crate) fn zero_key(&self) -> KeyNumber {
        self.zero_key
    }

    pub(crate) fn reference_key(&self) -> KeyNumber {
        self.reference_key
    }

    pub(crate) fn reference_frequency(&self) -> Frequency {
        self.reference_frequency
    }
}

impl Default for Reference {
    fn default() -> Self {
        let zero_key = KeyNumber::constant::<69>();
        Self {
            zero_key,
            reference_key: zero_key,
            reference_frequency: Frequency::CONCERT_A4,
        }
    }
}
