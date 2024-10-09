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
use crate::resources::RESOURCE_DIR;
use crate::scl_file::SclFile;
use crate::types::KeyNumber;
use anyhow::{anyhow, bail, Result};
use std::fs::read_to_string;
use std::path::Path;

#[allow(unused)]
pub(crate) fn read_expected_frequencies<P: AsRef<Path>>(path: P) -> Result<Vec<f64>> {
    let path = path.as_ref();
    let file = RESOURCE_DIR.get_file(path).ok_or_else(|| {
        anyhow!(
            "Expected frequency file {path} could not be opened",
            path = path.display()
        )
    })?;
    let s = file.contents_utf8().ok_or_else(|| {
        anyhow!(
            "Could not decode frequency file {path} as UTF-8",
            path = path.display()
        )
    })?;

    s.lines()
        .filter_map(|line| {
            let temp = line.trim();
            if temp.is_empty() {
                None
            } else {
                Some(temp)
            }
        })
        .map(|line| line.parse::<f64>().map_err(|e| anyhow!(e)))
        .collect::<Result<Vec<_>>>()
}

#[allow(unused)]
pub(crate) fn read_test_syx_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let path = path.as_ref();
    Ok(RESOURCE_DIR
        .get_file(path)
        .ok_or_else(|| anyhow!("Could not load tuning dump {path}", path = path.display()))?
        .contents()
        .to_vec())
}

#[allow(unused)]
pub(crate) fn read_test_scl_file<P: AsRef<Path>>(path: P) -> Result<SclFile> {
    let path = path.as_ref();
    let file = RESOURCE_DIR
        .get_file(path)
        .ok_or_else(|| anyhow!("Could not load .scl file {path}", path = path.display()))?;
    let s = file.contents_utf8().ok_or_else(|| {
        anyhow!(
            "Could not convert contents of {path} to string",
            path = path.display()
        )
    })?;
    s.parse::<SclFile>()
}

#[allow(unused)]
pub(crate) fn read_scala_tuning_dump<P: AsRef<Path>>(
    path: P,
    includes_header: bool,
) -> Result<Vec<(KeyNumber, Frequency)>> {
    let path = path.as_ref();
    scala_tuning_dump_from_str(&read_to_string(path)?, includes_header)
}

#[allow(unused)]
pub(crate) fn scala_tuning_dump_from_str(
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
    }

    let mut next_key = KeyNumber::ZERO;
    let mut mappings = Vec::new();
    for line in iter {
        let Some((prefix, suffix)) = line.split_once(':') else {
            bail!("Invalid Scala tuning dump (key not found)",);
        };

        let key = prefix.parse::<KeyNumber>()?;
        if key.to_u8() < next_key.to_u8() {
            bail!("Invalid Scala tuning dump (unexpected key {key})",);
        }

        let Some((prefix, _)) = suffix.trim_start().split_once(' ') else {
            bail!("Invalid Scala tuning dump (frequency not found)",);
        };

        mappings.push((key, Frequency(prefix.parse::<f64>()?)));
        next_key = key;
    }

    Ok(mappings)
}
