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

use crate::frequency::Frequency;
use crate::key_mappings::KeyMappings;
use crate::types::KeyNumber;
use anyhow::{bail, Result};

#[derive(Clone, Debug)]
pub(crate) struct KeyboardMapping {
    start_key: KeyNumber,
    end_key: KeyNumber,
    zero_key: KeyNumber,
    reference_key: KeyNumber,
    reference_frequency: Frequency,
    key_mappings: KeyMappings,
}

impl KeyboardMapping {
    pub(crate) fn new(
        start_key: KeyNumber,
        end_key: KeyNumber,
        zero_key: KeyNumber,
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
            zero_key,
            reference_key,
            reference_frequency,
            key_mappings,
        })
    }

    pub(crate) fn new_full(
        zero_key: KeyNumber,
        reference_key: KeyNumber,
        reference_frequency: Frequency,
        key_mappings: KeyMappings,
    ) -> Result<Self> {
        Self::new(
            KeyNumber::ZERO,
            KeyNumber::MAX,
            zero_key,
            reference_key,
            reference_frequency,
            key_mappings,
        )
    }

    pub(crate) fn new_full_linear(
        zero_key: KeyNumber,
        reference_key: KeyNumber,
        reference_frequency: Frequency,
    ) -> Result<Self> {
        Self::new_full(
            zero_key,
            reference_key,
            reference_frequency,
            KeyMappings::Linear,
        )
    }

    pub(crate) const fn start_key(&self) -> &KeyNumber {
        &self.start_key
    }

    pub(crate) const fn end_key(&self) -> &KeyNumber {
        &self.end_key
    }

    pub(crate) const fn zero_key(&self) -> &KeyNumber {
        &self.zero_key
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
