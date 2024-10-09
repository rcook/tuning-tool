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
use crate::num::round_default_scale;
use crate::semitones::Semitones;
use crate::types::{Lsb, Msb};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct MtsEntry {
    pub(crate) note_number: NoteNumber,
    pub(crate) msb: Msb,
    pub(crate) lsb: Lsb,
}

impl MtsEntry {
    // c.f. mtsBytesToMts
    pub(crate) fn to_semitones(&self) -> Semitones {
        fn make_14_bit(msb: Msb, lsb: Lsb) -> u16 {
            ((msb.to_u8() as u16) << 7) + lsb.to_u8() as u16
        }

        let lsb = if self.note_number.is_max() && self.lsb.is_max() {
            Lsb::constant::<0x7e>()
        } else {
            self.lsb
        };

        let fine = make_14_bit(self.msb, lsb) as f64 / 0x4000 as f64;
        Semitones(self.note_number.to_u8() as f64 + fine)
    }

    // c.f. mtsBytesToFrequency
    pub(crate) fn to_frequency(&self) -> Frequency {
        let mts = self.to_semitones();
        let frequency = mts.to_frequency();
        Frequency(round_default_scale(frequency.0))
    }
}

#[cfg(test)]
mod tests {
    use crate::frequency::Frequency;
    use crate::mts_entry::MtsEntry;
    use crate::note_number::NoteNumber;
    use crate::types::{Lsb, Msb};
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case(8.175799f64, (0, 0, 0))]
    #[case(8.175828f64, (0, 0, 1))]
    #[case(8.661957f64, (1, 0, 0))]
    #[case(16.351598f64, (12, 0, 0))]
    #[case(261.625565f64, (60, 0, 0))]
    #[case(277.182631f64, (61, 0, 0))]
    #[case(439.998449f64, (68, 127, 127))]
    #[case(440f64, (69, 0, 0))]
    #[case(440.001551f64, (69, 0, 1))]
    #[case(8372.01809f64, (120, 0, 0))]
    #[case(8372.047605f64, (120, 0, 1))]
    #[case(12543.853951f64, (127, 0, 0))]
    #[case(12543.898175f64, (127, 0, 1))]
    #[case(13289.656616f64, (127, 127, 126))]
    #[case(255.999612f64, (59, 79, 106))]
    #[case(441.999414f64, (69, 10, 6))]
    //#[case(439.998449f64, (68, 199, 199))]
    //#[case(13289.656616f64, (199, 199, 199))]
    fn to_frequency(#[case] expected: f64, #[case] input: (u8, u8, u8)) -> Result<()> {
        let input = MtsEntry {
            note_number: NoteNumber::try_from(input.0)?,
            msb: Msb::try_from(input.1)?,
            lsb: Lsb::try_from(input.2)?,
        };
        let expected = Frequency(expected);
        assert_eq!(expected.0, input.to_frequency().0);
        Ok(())
    }
}
