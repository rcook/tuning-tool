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
use crate::fs::read_to_string_lossy;
use crate::key_mapping::KeyMapping;
use crate::key_mappings::KeyMappings;
use crate::keyboard_mapping::KeyboardMapping;
use crate::reference::Reference;
use crate::types::KeyNumber;
use anyhow::bail;
use anyhow::{Error, Result};
use log::trace;
use std::path::Path;
use std::result::Result as StdResult;
use std::str::FromStr;
use tuning_tool_lib::u7::U7;

macro_rules! read_str {
    ($iter: expr) => {
        $iter
            .next()
            .ok_or_else(|| ::anyhow::anyhow!("Stream is exhausted"))?
    };
}

fn read<'a, U, I>(iter: &mut I) -> Result<U>
where
    U: U7,
    I: Iterator<Item = &'a str>,
{
    let s = read_str!(iter);
    let value = s.parse::<u8>()?;
    Ok(value.try_into()?)
}

macro_rules! read_f64 {
    ($iter: expr) => {{
        let s = read_str!($iter);
        let value = s.parse::<f64>()?;
        value
    }};
}

macro_rules! read_usize {
    ($iter: expr) => {{
        let s = read_str!($iter);
        let value = s.parse::<usize>()?;
        value
    }};
}

#[derive(Debug)]
pub(crate) struct KbmFile {
    _size: usize,
    _equave_degree: usize,
    keyboard_mapping: KeyboardMapping,
}

impl KbmFile {
    pub(crate) fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        trace!("Reading .kbm file {path}", path = path.as_ref().display());
        read_to_string_lossy(path)?.parse()
    }

    pub(crate) const fn keyboard_mapping(&self) -> &KeyboardMapping {
        &self.keyboard_mapping
    }
}

impl FromStr for KbmFile {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        trace!("Content is [[{s}]]");

        let mut lines = s.lines().filter_map(|line| {
            let s = line.trim();
            if s.is_empty() || s.starts_with("!") {
                None
            } else {
                Some(s)
            }
        });

        let size = read_str!(lines).parse::<usize>()?;
        if size > 127 {
            bail!("Invalid size")
        }

        // Start of MIDI key range
        let start_key = read::<KeyNumber, _>(&mut lines)?;
        trace!("Parsed start key {start_key}");

        // End of MIDI key range
        let end_key = read::<KeyNumber, _>(&mut lines)?;
        trace!("Parsed end key {end_key}");

        // Middle key where 1/1 note is mapped
        let zero_key = read::<KeyNumber, _>(&mut lines)?;
        trace!("Parsed zero key {zero_key}");

        // Key where reference frequency goes
        let reference_key = read::<KeyNumber, _>(&mut lines)?;
        trace!("Parsed reference key {reference_key}");

        // Reference frequency (e.g. 440 Hz)
        let reference_frequency = Frequency(read_f64!(lines));
        trace!("Parsed reference frequency {reference_frequency}");

        // Scale interval between adjacent repeating patterns
        let equave_degree = read_usize!(lines);
        trace!("Parsed equave degree {equave_degree}");

        let mut is_linear = true;
        let mut key_mappings = Vec::with_capacity(size);
        for i in 0..size {
            let s = read_str!(lines);
            let key_mapping = if s == "x" {
                is_linear = false;
                KeyMapping::Unmapped
            } else {
                let degree = s.parse()?;
                if is_linear && degree != i {
                    is_linear = false;
                }
                KeyMapping::Degree(degree)
            };
            trace!("Parsed key mapping {key_mapping}");
            key_mappings.push(key_mapping);
        }

        if lines.next().is_some() {
            bail!("Invalid .kbm file")
        }

        let key_mappings = if is_linear {
            KeyMappings::Linear
        } else {
            KeyMappings::Custom(key_mappings)
        };

        let reference = Reference::new(zero_key, reference_key, reference_frequency);
        let keyboard_mapping = KeyboardMapping::new(start_key, end_key, &reference, key_mappings)?;

        Ok(Self {
            _size: size,
            _equave_degree: equave_degree,
            keyboard_mapping,
        })
    }
}
