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
use std::path::Path;

#[allow(unused)]
pub(crate) fn read_expected_frequencies<P: AsRef<Path>>(path: P) -> Vec<f64> {
    read_resource_utf8(path.as_ref())
        .lines()
        .filter_map(|line| {
            let temp = line.trim();
            if temp.is_empty() {
                None // grcov-excl-line
            } else {
                Some(temp)
            }
        })
        .map(|line| line.parse::<f64>().expect("Content must be valid"))
        .collect::<Vec<_>>()
}

#[allow(unused)]
pub(crate) fn read_syx_file<P: AsRef<Path>>(path: P) -> Vec<u8> {
    read_resource_bytes(path.as_ref())
}

#[allow(unused)]
pub(crate) fn read_scl_file<P: AsRef<Path>>(path: P) -> SclFile {
    read_resource_utf8(path.as_ref())
        .parse::<SclFile>()
        .expect("Must be valid .scl file")
}

#[allow(unused)]
pub(crate) fn read_scala_tuning_dump<P: AsRef<Path>>(
    path: P,
    includes_header: bool,
) -> Vec<(KeyNumber, Frequency)> {
    let s = read_resource_utf8(path.as_ref());

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
            panic!("Invalid Scala tuning dump (key not found)"); // grcov-excl-line
        };

        let Ok(key) = prefix.parse::<KeyNumber>() else {
            panic!("Failed to parse key number"); // grcov-excl-line
        };

        if key.to_u8() < next_key.to_u8() {
            panic!("Invalid Scala tuning dump (unexpected key {key})"); // grcov-excl-line
        }

        let Some((prefix, _)) = suffix.trim_start().split_once(' ') else {
            panic!("Invalid Scala tuning dump (frequency not found)"); // grcov-excl-line
        };

        let Ok(frequency) = prefix.parse::<f64>() else {
            panic!("Failed to parse frequency"); // grcov-excl-line
        };

        mappings.push((key, Frequency(frequency)));
        next_key = key;
    }

    mappings
}

fn read_resource_utf8(path: &Path) -> String {
    let Some(file) = RESOURCE_DIR.get_file(path) else {
        panic!("Test data file {path} not found", path = path.display()); // grcov-excl-line
    };

    let Some(s) = file.contents_utf8() else {
        // grcov-excl-start
        panic!(
            "Test data file could not be decoded {path} as UTF-8",
            path = path.display()
        );
        // grcov-excl-stop
    };

    String::from(s)
}

fn read_resource_bytes(path: &Path) -> Vec<u8> {
    let Some(file) = RESOURCE_DIR.get_file(path) else {
        panic!("Test data file {path} not found", path = path.display()); // grcov-excl-line
    };

    file.contents().to_vec()
}
