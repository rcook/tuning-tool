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
use crate::note_number::NoteNumber;
use anyhow::{bail, Error};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::str::FromStr;

include!(concat!(env!("OUT_DIR"), "/midi_note_generated.rs"));

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct MidiNote {
    note_number: NoteNumber,
    name: &'static str,
    is_natural: bool,
    frequency: Frequency,
}

impl MidiNote {
    #[allow(unused)]
    pub(crate) const ALL: [MidiNote; 128] = ALL;

    #[allow(unused)]
    pub(crate) const fn note_number(&self) -> NoteNumber {
        self.note_number
    }

    #[allow(unused)]
    pub(crate) const fn name(&self) -> &str {
        self.name
    }

    #[allow(unused)]
    pub(crate) const fn is_natural(&self) -> bool {
        self.is_natural
    }

    #[allow(unused)]
    pub(crate) const fn frequency(&self) -> Frequency {
        self.frequency
    }

    const fn new(
        note_number: NoteNumber,
        name: &'static str,
        is_natural: bool,
        frequency: Frequency,
    ) -> Self {
        Self {
            note_number,
            name,
            is_natural,
            frequency,
        }
    }
}

impl Display for MidiNote {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} ({} Hz)", self.note_number.to_u8(), self.frequency.0)
    }
}

impl FromStr for MidiNote {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        if let Ok(value) = s.parse::<NoteNumber>() {
            return Ok(MidiNote::ALL[value.to_u8() as usize]);
        }

        let temp = s.to_uppercase();
        for midi_note in Self::ALL {
            if temp == midi_note.name() {
                return Ok(midi_note);
            }
        }

        bail!("Invalid MIDI note {s}")
    }
}

#[cfg(test)]
mod tests {
    use crate::approx_eq::{assert_approx_eq, ApproxEq};
    use crate::midi_note::MidiNote;
    use crate::note_number::NoteNumber;
    use anyhow::Result;
    use rstest::rstest;

    #[test]
    fn basics() -> Result<()> {
        assert_eq!(128, MidiNote::ALL.len());
        for (i, midi_note) in MidiNote::ALL.iter().enumerate() {
            let note_number = NoteNumber::try_from(i as u8)?;
            let frequency = 440f64 * 2f64.powf((i as i32 - 69) as f64 / 12f64);
            assert_eq!(note_number, midi_note.note_number());
            assert_approx_eq!(frequency, midi_note.frequency().0, 0.0000001f64);
        }
        Ok(())
    }

    #[test]
    fn concert_a() {
        let midi_note = MidiNote::ALL[69];
        assert_eq!(69, midi_note.note_number().to_u8());
        assert_eq!("A4", midi_note.name());
        assert!(midi_note
            .frequency()
            .0
            .approx_eq_with_epsilon(440f64, 0.001));
    }

    #[rstest]
    #[case(MidiNote::ALL[0], "0")]
    #[case(MidiNote::ALL[60], "60")]
    #[case(MidiNote::ALL[69], "69")]
    #[case(MidiNote::ALL[127], "127")]
    #[case(MidiNote::ALL[69], "a4")]
    #[case(MidiNote::ALL[69], "A4")]
    #[case(MidiNote::ALL[70], "A#4")]
    #[case(MidiNote::ALL[0], "C-1")]
    #[case(MidiNote::ALL[12], "C0")]
    fn from_str(#[case] expected: MidiNote, #[case] input: &str) {
        assert_eq!(expected, input.parse().expect("Must succeed"));
    }
}
