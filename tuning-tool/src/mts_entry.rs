use crate::consts::U7_MAX;
use crate::frequency::Frequency;
use crate::note_number::NoteNumber;
use crate::num::round_default_scale;
use crate::semitones::Semitones;
use crate::yy::YY;
use crate::zz::ZZ;
use midly::num::u7;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct MtsEntry {
    pub(crate) note_number: NoteNumber,
    pub(crate) yy: YY,
    pub(crate) zz: ZZ,
}

impl MtsEntry {
    // c.f. mtsBytesToMts
    #[allow(unused)]
    pub(crate) fn to_semitones(&self) -> Semitones {
        let yy = self.yy.to_u8();
        let zz = self.zz.to_u8();
        let msb = if yy > 0x7f { 0x7f } else { yy };
        let mut lsb = zz;
        let note_number = u7::from_int_lossy(self.note_number.to_u8());

        if note_number == U7_MAX && lsb == U7_MAX {
            lsb = 0x7e;
        }

        let fine = (((msb as u16) << 7) + lsb as u16) as f64 / 0x4000 as f64;
        Semitones(note_number.as_int() as f64 + fine)
    }

    // c.f. mtsBytesToFrequency
    #[allow(unused)]
    pub(crate) fn to_frequency(&self) -> Frequency {
        let mts = self.to_semitones();
        let frequency = mts.to_frequency();
        Frequency(round_default_scale(frequency.0))
    }

    // c.f. mtsBytesToHex
    #[allow(unused)]
    pub(crate) fn to_hex(&self) -> String {
        format!(
            "{xx:02x}{yy:02x}{zz:02x}",
            xx = self.note_number,
            yy = self.yy,
            zz = self.zz
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::frequency::Frequency;
    use crate::mts_entry::MtsEntry;
    use crate::note_number::NoteNumber;
    use crate::yy::YY;
    use crate::zz::ZZ;
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
            yy: YY::try_from(input.1)?,
            zz: ZZ::try_from(input.2)?,
        };
        let expected = Frequency(expected);
        assert_eq!(expected.0, input.to_frequency().0);
        Ok(())
    }

    #[rstest]
    #[case("3c0000", (60, 0, 0))]
    #[case("450000", (69, 0, 0))]
    #[case("450a06", (69, 10, 6))]
    //#[case("457f06", (69, 240, 6))]
    //#[case("7f7f7f", (128, 255, 128))]
    #[case("7f7f7f", (127, 127, 127))]
    fn to_hex(#[case] expected: &str, #[case] input: (u8, u8, u8)) -> Result<()> {
        let input = MtsEntry {
            note_number: NoteNumber::try_from(input.0)?,
            yy: YY::try_from(input.1)?,
            zz: ZZ::try_from(input.2)?,
        };
        assert_eq!(expected, input.to_hex());
        Ok(())
    }
}
