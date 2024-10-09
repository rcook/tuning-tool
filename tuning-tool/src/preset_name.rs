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

use crate::types::Char7;
use anyhow::{bail, Error};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::str::FromStr;

const PRESET_NAME_LEN: usize = 16;

type PresetNameArray = [Char7; PRESET_NAME_LEN];

#[derive(Debug)]
pub(crate) struct PresetName(PresetNameArray);

impl PresetName {
    pub(crate) const LEN: usize = PRESET_NAME_LEN;

    pub(crate) const fn new(slice: PresetNameArray) -> Self {
        Self(slice)
    }

    pub(crate) const fn as_array(&self) -> &PresetNameArray {
        &self.0
    }
}

impl Display for PresetName {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let s = String::from_utf8_lossy(Char7::to_u8_slice(&self.0));
        write!(f, "{}", s)
    }
}

impl FromStr for PresetName {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        if !s.is_ascii() {
            bail!("Preset name is not ASCII")
        }
        if s.len() > Self::LEN {
            bail!("Invalid preset name")
        }

        // Terribly inefficient, but at least it's safe!
        let mut array = [Char7::ZERO; Self::LEN];
        for (i, c) in s.bytes().enumerate() {
            array[i] = c.try_into()?;
        }

        Ok(Self(array))
    }
}
