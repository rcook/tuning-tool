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
        if let Ok(midi_note) = s.parse::<MidiNote>() {
            return Ok(KeyNumber::from_u8_lossy(midi_note.note_number().to_u8()));
        }

        Err(format!("Invalid key {s}"))
    }

    if s.to_lowercase() == "default" {
        Ok(Reference::default())
    } else {
        let Some((prefix, suffix)) = s.split_once(',') else {
            return Err(format!("Invalid reference {s}"));
        };

        let zero_key = parse_key_number(prefix)?;

        let Some((prefix, suffix)) = suffix.split_once('=') else {
            return Err(format!("Invalid reference {s}"));
        };

        let reference_key = parse_key_number(prefix)?;

        let Ok(f) = suffix.parse() else {
            return Err(format!("Invalid reference {s}"));
        };

        let reference_frequency = Frequency(f);
        Ok(Reference::new(zero_key, reference_key, reference_frequency))
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::{parse_absolute_path, parse_reference};
    use crate::frequency::Frequency;
    use crate::reference::Reference;
    use crate::types::KeyNumber;
    use rstest::rstest;

    #[test]
    fn parse_absolute_path_basics() {
        assert!(parse_absolute_path("aaa/bbb/ccc")
            .expect("Must succeed")
            .is_absolute());
    }

    #[rstest]
    #[case(Reference::new(KeyNumber::constant::<48>(), KeyNumber::constant::<69>(), Frequency(440f64) ), "c3,a4=440")]
    #[case(Reference::new(KeyNumber::constant::<48>(), KeyNumber::constant::<69>(), Frequency(432f64) ), "c3,69=432")]
    fn parse_reference_basics(#[case] expected: Reference, #[case] input: &str) {
        assert_eq!(expected, parse_reference(input).expect("Must succeed"));
    }
}
