use crate::frequency::Frequency;
use crate::note_number::NoteNumber;
use crate::num::round_default_scale;
use crate::semitones::Semitones;

#[derive(Debug, PartialEq)]
pub(crate) struct MtsBytes {
    pub(crate) note_number: NoteNumber,
    pub(crate) yy: u8,
    pub(crate) zz: u8,
}

impl MtsBytes {
    // c.f. mtsBytesToMts
    pub(crate) fn to_semitones(&self) -> Semitones {
        let msb = if self.yy > 0x7f { 0x7f } else { self.yy };
        let mut lsb = self.zz;
        let note_number = if self.note_number.0 > 0x7f {
            0x7f
        } else {
            self.note_number.0 as u8
        };

        if note_number == 0x7f {
            if lsb >= 0x7f {
                lsb = 0x7e;
            }
        } else if lsb > 0x7f {
            lsb = 0x7f;
        }

        let fine = (((msb as u16) << 7) + lsb as u16) as f64 / 0x4000 as f64;
        Semitones(note_number as f64 + fine)
    }

    // c.f. mtsBytesToFrequency
    pub(crate) fn to_frequency(&self) -> Frequency {
        let mts = self.to_semitones();
        println!("mts={:?}", mts.0);
        let frequency = mts.to_frequency();
        Frequency(round_default_scale(frequency.0))
    }

    // c.f. mtsBytesToHex
    pub(crate) fn to_hex(&self) -> String {
        format!(
            "{xx:02x}{yy:02x}{zz:02x}",
            xx = self.note_number.0.clamp(0, 127),
            yy = self.yy.clamp(0, 127),
            zz = self.zz.clamp(0, 127)
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::frequency::Frequency;
    use crate::mts_bytes::MtsBytes;
    use crate::note_number::NoteNumber;
    use rstest::rstest;

    #[rstest]
    #[case(Frequency(8.175799f64), MtsBytes { note_number: NoteNumber(0), yy: 0, zz: 0 })]
    #[case(Frequency(8.175828f64), MtsBytes { note_number: NoteNumber(0), yy: 0, zz: 1 })]
    #[case(Frequency(8.661957f64), MtsBytes { note_number: NoteNumber(1), yy: 0, zz: 0 })]
    #[case(Frequency(16.351598f64), MtsBytes { note_number: NoteNumber(12), yy: 0, zz: 0 })]
    #[case(Frequency(261.625565f64), MtsBytes { note_number: NoteNumber(60), yy: 0, zz: 0 })]
    #[case(Frequency(277.182631f64), MtsBytes { note_number: NoteNumber(61), yy: 0, zz: 0 })]
    #[case(Frequency(439.998449f64), MtsBytes { note_number: NoteNumber(68), yy: 127, zz: 127 })]
    #[case(Frequency(440f64), MtsBytes { note_number: NoteNumber(69), yy: 0, zz: 0 })]
    #[case(Frequency(440.001551f64), MtsBytes { note_number: NoteNumber(69), yy: 0, zz: 1 })]
    #[case(Frequency(8372.01809f64), MtsBytes { note_number: NoteNumber(120), yy: 0, zz: 0 })]
    #[case(Frequency(8372.047605f64), MtsBytes { note_number: NoteNumber(120), yy: 0, zz: 1 })]
    #[case(Frequency(12543.853951f64), MtsBytes { note_number: NoteNumber(127), yy: 0, zz: 0 })]
    #[case(Frequency(12543.898175f64), MtsBytes { note_number: NoteNumber(127), yy: 0, zz: 1 })]
    #[case(Frequency(13289.656616f64), MtsBytes { note_number: NoteNumber(127), yy: 127, zz: 126 })]
    #[case(Frequency(255.999612f64), MtsBytes { note_number: NoteNumber(59), yy: 79, zz: 106 })]
    #[case(Frequency(441.999414f64), MtsBytes { note_number: NoteNumber(69), yy: 10, zz: 6 })]
    #[case(Frequency(439.998449f64), MtsBytes { note_number: NoteNumber(68), yy: 199, zz: 199 })]
    #[case(Frequency(13289.656616f64), MtsBytes { note_number: NoteNumber(199), yy: 199, zz: 199 })]
    fn to_frequency(#[case] expected: Frequency, #[case] input: MtsBytes) {
        assert_eq!(expected.0, input.to_frequency().0);
    }

    #[rstest]
    #[case("3c0000", MtsBytes { note_number: NoteNumber(60), yy: 0, zz: 0 })]
    #[case("450000", MtsBytes { note_number: NoteNumber(69), yy: 0, zz: 0 })]
    #[case("450a06", MtsBytes { note_number: NoteNumber(69), yy: 10, zz: 6 })]
    #[case("457f06", MtsBytes { note_number: NoteNumber(69), yy: 240, zz: 6 })]
    #[case("7f7f7f", MtsBytes { note_number: NoteNumber(128), yy: 255, zz: 128 })]
    #[case("7f7f7f", MtsBytes { note_number: NoteNumber(127), yy: 127, zz: 127 })]
    fn to_hex(#[case] expected: &str, #[case] input: MtsBytes) {
        assert_eq!(expected, input.to_hex());
    }
}
