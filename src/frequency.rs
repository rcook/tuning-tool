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
    #[case(Semitones(0f64), Frequency(8.175799f64))]
    #[case(Semitones(0.000062f64), Frequency(8.175828f64))]
    #[case(Semitones(1f64), Frequency(8.661957f64))]
    #[case(Semitones(12f64), Frequency(16.351598f64))]
    #[case(Semitones(60f64), Frequency(261.625565f64))]
    #[case(Semitones(61f64), Frequency(277.182631f64))]
    #[case(Semitones(68.999939f64), Frequency(439.998449f64))]
    #[case(Semitones(69f64), Frequency(440f64))]
    #[case(Semitones(69.000061f64), Frequency(440.001551f64))]
    #[case(Semitones(120f64), Frequency(8372.01809f64))]
    #[case(Semitones(120.000061f64), Frequency(8372.047605f64))]
    #[case(Semitones(127f64), Frequency(12543.853951f64))]
    #[case(Semitones(127.000061f64), Frequency(12543.898175f64))]
    #[case(Semitones(127.999878f64), Frequency(13289.656616f64))]
    fn to_semitones(#[case] expected: Semitones, #[case] input: Frequency) {
        assert_eq!(expected.0, input.to_semitones().0);
    }

    #[rstest]
    #[case(Semitones(127.999878f64), Frequency(13289.656616f64), true)]
    #[case(Semitones(0f64), Frequency(-100f64), false)]
    #[case(Semitones(127.999878f64), Frequency(14080f64), false)]
    #[case(Semitones(127.999878f64), Frequency(28980f64), false)]
    #[case(Semitones(0f64), Frequency(-100f64), true)]
    #[case(Semitones(129f64), Frequency(14080f64), true)]
    #[case(Semitones(141.496923f64), Frequency(28980f64), true)]
    fn to_semitones_with_ignore_limit(
        #[case] expected: Semitones,
        #[case] input: Frequency,
        #[case] ignore_limit: bool,
    ) {
        assert_eq!(
            expected.0,
            input.to_semitones_with_ignore_limit(ignore_limit).0
        );
    }

    #[rstest]
    #[case(MtsBytes { note_number: NoteNumber(0), yy: 0, zz: 0 }, Frequency(8.175799f64))]
    #[case(MtsBytes { note_number: NoteNumber(0), yy: 0, zz: 1 }, Frequency(8.175828f64))]
    #[case(MtsBytes { note_number: NoteNumber(1), yy: 0, zz: 0 }, Frequency(8.661957f64))]
    #[case(MtsBytes { note_number: NoteNumber(12), yy: 0, zz: 0 }, Frequency(16.351598f64))]
    #[case(MtsBytes { note_number: NoteNumber(60), yy: 0, zz: 0 }, Frequency(261.625565f64))]
    #[case(MtsBytes { note_number: NoteNumber(61), yy: 0, zz: 0 }, Frequency(277.182631f64))]
    #[case(MtsBytes { note_number: NoteNumber(68), yy: 127, zz: 127 }, Frequency(439.998449f64))]
    #[case(MtsBytes { note_number: NoteNumber(69), yy: 0, zz: 0 }, Frequency(440f64))]
    #[case(MtsBytes { note_number: NoteNumber(69), yy: 0, zz: 1 }, Frequency(440.001551f64))]
    #[case(MtsBytes { note_number: NoteNumber(120), yy: 0, zz: 0 }, Frequency(8372.01809f64))]
    #[case(MtsBytes { note_number: NoteNumber(120), yy: 0, zz: 1 }, Frequency(8372.047605f64))]
    #[case(MtsBytes { note_number: NoteNumber(127), yy: 0, zz: 0 }, Frequency(12543.853951f64))]
    #[case(MtsBytes { note_number: NoteNumber(127), yy: 0, zz: 1 }, Frequency(12543.898175f64))]
    #[case(MtsBytes { note_number: NoteNumber(127), yy: 127, zz: 126 }, Frequency(13289.656616f64))]
    #[case(MtsBytes { note_number: NoteNumber(59), yy: 79, zz: 106 }, Frequency(255.999612f64))]
    #[case(MtsBytes { note_number: NoteNumber(59), yy: 79, zz: 106 }, Frequency(256f64))]
    #[case(MtsBytes { note_number: NoteNumber(69), yy: 10, zz: 6 }, Frequency(441.999414f64))]
    #[case(MtsBytes { note_number: NoteNumber(69), yy: 10, zz: 6 }, Frequency(442f64))]
    #[case(MtsBytes { note_number: NoteNumber(0), yy: 0, zz: 0 }, Frequency(-1f64))]
    #[case(MtsBytes { note_number: NoteNumber(127), yy: 127, zz: 126 }, Frequency(14000f64))]
    fn to_mts_bytes(#[case] expected: MtsBytes, #[case] input: Frequency) {
        assert_eq!(expected, input.to_mts_bytes())
    }

    #[test]
    fn to_note_number() {
        let (note_number, cent_offset) = Frequency(261.625565f64).to_note_number();
        assert_eq!(60, note_number.0);
        assert!(cent_offset.0.approx_eq_with_epsilon(0f64, 0.00001f64));
    }
}
