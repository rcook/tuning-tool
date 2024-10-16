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

#![cfg(test)]

use crate::frequency::Frequency;
use crate::types::KeyNumber;
use anyhow::{bail, Result};

macro_rules! include_resource_bytes {
    ($path: expr) => {
        std::include_bytes!(std::concat!(
            std::env!("CARGO_MANIFEST_DIR"),
            "/../resources/test/",
            $path
        ))
    };
}
pub(crate) use include_resource_bytes;

macro_rules! include_resource_str {
    ($path: expr) => {
        std::include_str!(std::concat!(
            std::env!("CARGO_MANIFEST_DIR"),
            "/../resources/test/",
            $path
        ))
    };
}
pub(crate) use include_resource_str;

pub(crate) fn parse_frequencies(s: &str) -> Result<Vec<f64>> {
    s.lines()
        .filter_map(|line| {
            let temp = line.trim();
            if temp.is_empty() {
                None // grcov-excl-line
            } else {
                Some(temp)
            }
        })
        .map(|line| Ok(line.parse::<f64>()?))
        .collect::<Result<Vec<_>>>()
}

pub(crate) fn parse_scala_tuning_dump(
    s: &str,
    includes_header: bool,
) -> Result<Vec<(KeyNumber, Frequency)>> {
    let mut iter = s.lines().map(|line| line.trim());

    if includes_header {
        for line in iter.by_ref() {
            if line == "|" {
                break;
            }
        }
    } // grcov-excl-line

    let mut next_key = KeyNumber::ZERO;
    let mut mappings = Vec::new();
    for line in iter {
        let Some((prefix, suffix)) = line.split_once(':') else {
            bail!("Invalid Scala tuning dump (key not found)"); // grcov-excl-line
        };

        let Ok(key) = prefix.parse::<KeyNumber>() else {
            bail!("Failed to parse key number"); // grcov-excl-line
        };

        if key.to_u8() < next_key.to_u8() {
            bail!("Invalid Scala tuning dump (unexpected key {key})"); // grcov-excl-line
        }

        let Some((prefix, _)) = suffix.trim_start().split_once(' ') else {
            bail!("Invalid Scala tuning dump (frequency not found)"); // grcov-excl-line
        };

        let Ok(frequency) = prefix.parse::<f64>() else {
            bail!("Failed to parse frequency"); // grcov-excl-line
        };

        mappings.push((key, Frequency(frequency)));
        next_key = key;
    }

    Ok(mappings)
}
