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
use crate::midi_note::MidiNote;
use crate::reference::Reference;
use crate::types::KeyNumber;
use path_absolutize::Absolutize;
use std::path::PathBuf;
use std::result::Result as StdResult;

pub(crate) fn parse_absolute_path(s: &str) -> StdResult<PathBuf, String> {
    PathBuf::from(s)
        .absolutize()
        .map_err(|_| String::from("Invalid path"))
        .map(|p| p.to_path_buf())
}

pub(crate) fn parse_reference(s: &str) -> StdResult<Reference, String> {
    fn parse_key_number(s: &str) -> StdResult<KeyNumber, String> {
        if let Ok(key_number) = s.parse() {
            return Ok(key_number);
        }

        let temp = s.to_uppercase();
        for midi_note in MidiNote::ALL {
            if temp == midi_note.name() {
                return Ok(KeyNumber::from_u8_lossy(midi_note.note_number().to_u8()));
            }
        }

        Err(format!("Invalid key {s}"))
    }

    if s.to_lowercase() == "default" {
        Ok(Reference::default())
    } else {
        let parts = s.split(',').collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(String::from("Invalid reference"));
        }

        let zero_key = parse_key_number(parts[0])?;
        let reference_key = parse_key_number(parts[1])?;
        let Ok(f) = parts[2].parse() else {
            return Err(String::from("Invalid reference frequency"));
        };
        let reference_frequency = Frequency(f);
        Ok(Reference::new(zero_key, reference_key, reference_frequency))
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::parse_absolute_path;

    #[test]
    fn basics() {
        assert!(parse_absolute_path("aaa/bbb/ccc")
            .expect("Must succeed")
            .is_absolute());
    }
}
