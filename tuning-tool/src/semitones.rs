use crate::consts::{U7_MAX, U7_ZERO};
use crate::frequency::Frequency;
use crate::mts_entry::MtsEntry;
use crate::note_number::NoteNumber;
use anyhow::Result;
use midly::num::u7;

pub(crate) struct Semitones(pub(crate) f64);

impl Semitones {
    pub(crate) const MAX: Self = Self(127.999878f64);

    // c.f. mtsToMtsBytes
    pub(crate) fn to_mts_entry(&self) -> Result<MtsEntry> {
        if self.0 <= 0f64 {
            return Ok(MtsEntry {
                note_number: NoteNumber::ZERO,
                yy: U7_ZERO,
                zz: U7_ZERO,
            });
        }

        if self.0 > 127.999878f64 {
            return Ok(MtsEntry {
                note_number: NoteNumber::MAX,
                yy: U7_MAX,
                zz: u7::from_int_lossy(0x7e),
            });
        }

        let note_number = self.0.trunc();
        let fine = ((0x4000 as f64) * (self.0 - note_number)).round() as u16;

        let yy = u7::from_int_lossy(((fine >> 7) & 0x7f) as u8);
        let zz = u7::from_int_lossy((fine & 0x7f) as u8);
        Ok(MtsEntry {
            note_number: NoteNumber::try_from(note_number as u8)?,
            yy,
            zz,
        })
    }

    pub(crate) fn to_frequency(&self) -> Frequency {
        let temp: f64 = NoteNumber::A4.to_u8() as f64;
        Frequency(Frequency::A4.0 * 2f64.powf((self.0 - temp) / 12f64))
    }
}

#[cfg(test)]
mod tests {
    use crate::mts_entry::MtsEntry;
    use crate::note_number::NoteNumber;
    use crate::semitones::Semitones;
    use anyhow::Result;
    use midly::num::u7;
    use rstest::rstest;

    #[rstest]
    #[case((0, 0, 0), 0f64)]
    #[case((0, 0, 1), 0.00006f64)]
    #[case((1, 0, 0), 1f64)]
    #[case((12, 0, 0), 12f64)]
    #[case((60, 0, 0), 60f64)]
    #[case((61, 0, 0), 61f64)]
    #[case((68, 127, 127), 68.99994f64)]
    #[case((69, 0, 0), 69f64)]
    #[case((69, 0, 1), 69.000061f64)]
    #[case((120, 0, 0), 120f64)]
    #[case((120, 0, 1), 120.000061f64)]
    #[case((127, 0, 0), 127f64)]
    #[case((127, 0, 1), 127.000061f64)]
    #[case((127, 127, 126), 127.999878f64)]
    #[case((0, 0, 0), -1f64)]
    #[case((127, 127, 126), 127.9999f64)]
    #[case((127, 127, 126), 128f64)]
    fn to_mts_entry(#[case] expected: (u8, u8, u8), #[case] input: f64) -> Result<()> {
        let input = Semitones(input);
        let expected = MtsEntry {
            note_number: NoteNumber::try_from(expected.0)?,
            yy: u7::from_int_lossy(expected.1),
            zz: u7::from_int_lossy(expected.2),
        };
        assert_eq!(expected, input.to_mts_entry()?);
        Ok(())
    }
}
