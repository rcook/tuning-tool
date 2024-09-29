use crate::cent_offset::CentOffset;
use crate::mts_bytes::MtsBytes;
use crate::note_number::NoteNumber;
use crate::num::round_default_scale;
use crate::ratio::Ratio;
use crate::semitones::Semitones;

#[derive(Clone, Copy)]
pub(crate) struct Frequency(pub(crate) f64);

impl Frequency {
    pub(crate) const A4: Self = Self(440f64);
    pub(crate) const MIDDLE_C: Self = Self(261.625565f64);
    pub(crate) const MIN: Self = Self(8.175798915643707f64);
    pub(crate) const MAX: Self = Self(13289.656616f64);

    // c.f. frequencyToCentOffset
    pub(crate) fn to_cent_offset(&self) -> CentOffset {
        self.to_cent_offset_with_base_frequency(Self::A4)
    }

    pub(crate) fn to_cent_offset_with_base_frequency(
        &self,
        base_frequency: Frequency,
    ) -> CentOffset {
        CentOffset(Ratio(self.0 / base_frequency.0).to_cents().0)
    }

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

    // c.f. ftom
    pub(crate) fn to_note_number(&self) -> (NoteNumber, CentOffset) {
        let semitones = self.semitones_raw();
        let note_number = semitones.0.round();
        let cent_offset = (semitones.0 - note_number) * 100f64;
        (NoteNumber(note_number as i32), CentOffset(cent_offset))
    }

    pub(crate) fn to_mts_bytes(&self) -> MtsBytes {
        self.to_semitones().to_mts_bytes()
    }

    fn semitones_raw(&self) -> Semitones {
        Semitones((NoteNumber::A4.0 as f64) + 12f64 * (self.0 / Self::A4.0).log2())
    }
}

#[cfg(test)]
mod tests {
    use crate::approx_eq::ApproxEq;
    use crate::frequency::Frequency;
    use crate::mts_bytes::MtsBytes;
    use crate::note_number::NoteNumber;
    use crate::semitones::Semitones;
    use rstest::rstest;

    #[test]
    fn basics() {
        assert!(Frequency(220f64).to_cent_offset().0.approx_eq(-1200f64));
        assert!(Frequency(220f64)
            .to_cent_offset_with_base_frequency(Frequency(220f64))
            .0
            .approx_eq(0f64));
    }

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
    fn to_mts_bytes(#[case] expected: (u8, u8, u8), #[case] input: f64) {
        let input = Frequency(input);
        let expected = MtsBytes {
            note_number: NoteNumber(expected.0.into()),
            yy: expected.1,
            zz: expected.2,
        };
        assert_eq!(expected, input.to_mts_bytes())
    }

    #[test]
    fn to_note_number() {
        let (note_number, cent_offset) = Frequency(261.625565f64).to_note_number();
        assert_eq!(60, note_number.0);
        assert!(cent_offset.0.approx_eq_with_epsilon(0f64, 0.00001f64));
    }
}
