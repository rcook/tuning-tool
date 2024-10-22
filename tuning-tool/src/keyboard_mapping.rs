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

use crate::key_mappings::KeyMappings;
use crate::reference::Reference;
use crate::types::KeyNumber;
use anyhow::{bail, Result};

#[derive(Clone, Debug)]
pub(crate) struct KeyboardMapping {
    start_key: KeyNumber,
    end_key: KeyNumber,
    reference: Reference,
    key_mappings: KeyMappings,
}

impl KeyboardMapping {
    pub(crate) fn new(
        start_key: KeyNumber,
        end_key: KeyNumber,
        reference: &Reference,
        key_mappings: KeyMappings,
    ) -> Result<Self> {
        if end_key.checked_sub(start_key).is_none() {
            bail!("Invalid end key");
        }

        Ok(Self {
            start_key,
            end_key,
            reference: reference.clone(),
            key_mappings,
        })
    }

    pub(crate) fn new_full(reference: &Reference, key_mappings: KeyMappings) -> Result<Self> {
        Self::new(KeyNumber::ZERO, KeyNumber::MAX, reference, key_mappings)
    }

    pub(crate) fn new_full_linear(reference: &Reference) -> Result<Self> {
        Self::new_full(reference, KeyMappings::Linear)
    }

    pub(crate) const fn start_key(&self) -> &KeyNumber {
        &self.start_key
    }

    pub(crate) const fn end_key(&self) -> &KeyNumber {
        &self.end_key
    }

    pub(crate) const fn reference(&self) -> &Reference {
        &self.reference
    }

    pub(crate) const fn key_mappings(&self) -> &KeyMappings {
        &self.key_mappings
    }
}
