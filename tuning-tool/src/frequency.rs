use crate::mts_entry::MtsEntry;
use crate::note_number::NoteNumber;
use crate::num::round_default_scale;
use crate::semitones::Semitones;
use crate::types::f64_newtype;
use anyhow::Result;

f64_newtype!(Frequency, pub(crate));

impl Frequency {
    pub(crate) const A4: Self = Self(440f64);
    pub(crate) const MAX: Self = Self(13289.656616f64);

    #[cfg(test)]
    pub(crate) const MIN: Frequency = crate::midi_note::MidiNote::ALL[0].frequency();

    // c.f. ftomts
    pub(crate) fn to_semitones(&self) -> Semitones {
        self.to_semitones_with_ignore_limit(false)
    }

    pub(crate) fn to_semitones_with_ignore_limit(&self, ignore_limit: bool) -> Semitones {
        if self.0 <= 0f64 {
            return Semitones(0f64);
        }

        if self.0 > Self::MAX.0 && !ignore_limit {
            return Semitones::MAX;
        }

        let semitones = self.semitones_raw().0;
        let value = round_default_scale(semitones);
        Semitones(value)
    }

    pub(crate) fn to_mts_entry(&self) -> Result<MtsEntry> {
        self.to_semitones().to_mts_entry()
    }

    fn semitones_raw(&self) -> Semitones {
        Semitones((NoteNumber::A4.to_u8() as f64) + 12f64 * (self.0 / Self::A4.0).log2())
    }
}

#[cfg(test)]
mod tests {
    use crate::frequency::Frequency;
    use crate::mts_entry::MtsEntry;
    use crate::note_number::NoteNumber;
    use crate::semitones::Semitones;
    use crate::types::{Lsb, Msb};
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case(0f64, 8.175799f64)]
    #[case(0.000062f64, 8.175828f64)]
    #[case(1f64, 8.661957f64)]
    #[case(12f64, 16.351598f64)]
    #[case(60f64, 261.625565f64)]
    #[case(61f64, 277.182631f64)]
    #[case(68.999939f64, 439.998449f64)]
    #[case(69f64, 440f64)]
    #[case(69.000061f64, 440.001551f64)]
    #[case(120f64, 8372.01809f64)]
    #[case(120.000061f64, 8372.047605f64)]
    #[case(127f64, 12543.853951f64)]
    #[case(127.000061f64, 12543.898175f64)]
    #[case(127.999878f64, 13289.656616f64)]
    fn to_semitones(#[case] expected: f64, #[case] input: f64) {
        let input = Frequency(input);
        let expected = Semitones(expected);
        assert_eq!(expected.0, input.to_semitones().0);
    }

    #[rstest]
    #[case(127.999878f64, 13289.656616f64, true)]
    #[case(0f64, -100f64, false)]
    #[case(127.999878f64, 14080f64, false)]
    #[case(127.999878f64, 28980f64, false)]
    #[case(0f64, -100f64, true)]
    #[case(129f64, 14080f64, true)]
    #[case(141.496923f64, 28980f64, true)]
    fn to_semitones_with_ignore_limit(
        #[case] expected: f64,
        #[case] input: f64,
        #[case] ignore_limit: bool,
    ) {
        let input = Frequency(input);
        let expected = Semitones(expected);
        assert_eq!(
            expected.0,
            input.to_semitones_with_ignore_limit(ignore_limit).0
        );
    }

    #[rstest]
    #[case((0, 0, 0), 8.175799f64)]
    #[case((0, 0, 1), 8.175828f64)]
    #[case((1, 0, 0), 8.661957f64)]
    #[case((12, 0, 0), 16.351598f64)]
    #[case((60, 0, 0), 261.625565f64)]
    #[case((61, 0, 0), 277.182631f64)]
    #[case((68, 127, 127), 439.998449f64)]
    #[case((69, 0, 0), 440f64)]
    #[case((69, 0, 1), 440.001551f64)]
    #[case((120, 0, 0), 8372.01809f64)]
    #[case((120, 0, 1), 8372.047605f64)]
    #[case((127, 0, 0), 12543.853951f64)]
    #[case((127, 0, 1), 12543.898175f64)]
    #[case((127, 127, 126), 13289.656616f64)]
    #[case((59, 79, 106), 255.999612f64)]
    #[case((59, 79, 106), 256f64)]
    #[case((69, 10, 6), 441.999414f64)]
    #[case((69, 10, 6), 442f64)]
    #[case((0, 0, 0), -1f64)]
    #[case((127, 127, 126), 14000f64)]
    fn to_mts_entry(#[case] expected: (u8, u8, u8), #[case] input: f64) -> Result<()> {
        let input = Frequency(input);
        let expected = MtsEntry {
            note_number: NoteNumber::try_from(expected.0)?,
            msb: Msb::try_from(expected.1)?,
            lsb: Lsb::try_from(expected.2)?,
        };
        assert_eq!(expected, input.to_mts_entry()?);
        Ok(())
    }
}
