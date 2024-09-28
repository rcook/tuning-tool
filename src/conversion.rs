use crate::cents::Cents;
use crate::mts_bytes::MtsBytes;
use crate::num::round_default_scale;
use crate::ratio::Ratio;
use crate::semitones::Semitones;

#[cfg(test)]
mod tests {
    use crate::approx_eq::ApproxEq;
    use crate::cent_offset::CentOffset;
    use crate::cents::Cents;
    use crate::frequency::Frequency;
    use crate::mts_bytes::MtsBytes;
    use crate::note_number::NoteNumber;
    use crate::ratio::Ratio;
    use crate::semitones::Semitones;
    use rstest::rstest;

    const FREQUENCY_EPSILON: Frequency = Frequency(0.000001f64);
    const CENT_OFFSET_EPSILON: CentOffset = CentOffset(0.00001f64);

    #[test]
    fn basics() {
        assert!(Ratio(3f64 / 2f64).to_cents().0.approx_eq(701.9550008653874));
        assert!(Cents(1200f64).to_ratio().0.approx_eq(2f64));
        assert!(Frequency(220f64).to_cent_offset().0.approx_eq(-1200f64));
        assert!(Frequency(220f64)
            .to_cent_offset_with_base_frequency(Frequency(220f64))
            .0
            .approx_eq(0f64));
        assert!(CentOffset(1200f64).to_frequency().0.approx_eq(880f64));
        assert!(CentOffset(1200f64)
            .to_frequency_with_base_frequency(Frequency(220f64))
            .0
            .approx_eq(440f64));
        assert!(NoteNumber(60)
            .to_frequency()
            .0
            .approx_eq_with_epsilon(261.625565, FREQUENCY_EPSILON.0));
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
    fn frequency_to_semitones(#[case] expected: Semitones, #[case] input: Frequency) {
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
    fn frequency_to_semitones_with_ignore_limit(
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
    #[case(MtsBytes { note_number: NoteNumber(0), yy: 0, zz: 0 }, Semitones(0f64))]
    #[case(MtsBytes { note_number: NoteNumber(0), yy: 0, zz: 1 }, Semitones(0.00006f64))]
    #[case(MtsBytes { note_number: NoteNumber(1), yy: 0, zz: 0 }, Semitones(1f64))]
    #[case(MtsBytes { note_number: NoteNumber(12), yy: 0, zz: 0 }, Semitones(12f64))]
    #[case(MtsBytes { note_number: NoteNumber(60), yy: 0, zz: 0 }, Semitones(60f64))]
    #[case(MtsBytes { note_number: NoteNumber(61), yy: 0, zz: 0 }, Semitones(61f64))]
    #[case(MtsBytes { note_number: NoteNumber(68), yy: 127, zz: 127 }, Semitones(68.99994f64))]
    #[case(MtsBytes { note_number: NoteNumber(69), yy: 0, zz: 0 }, Semitones(69f64))]
    #[case(MtsBytes { note_number: NoteNumber(69), yy: 0, zz: 1 }, Semitones(69.000061f64))]
    #[case(MtsBytes { note_number: NoteNumber(120), yy: 0, zz: 0 }, Semitones(120f64))]
    #[case(MtsBytes { note_number: NoteNumber(120), yy: 0, zz: 1 }, Semitones(120.000061f64))]
    #[case(MtsBytes { note_number: NoteNumber(127), yy: 0, zz: 0 }, Semitones(127f64))]
    #[case(MtsBytes { note_number: NoteNumber(127), yy: 0, zz: 1 }, Semitones(127.000061f64))]
    #[case(MtsBytes { note_number: NoteNumber(127), yy: 127, zz: 126 }, Semitones(127.999878f64))]
    #[case(MtsBytes { note_number: NoteNumber(0), yy: 0, zz: 0 }, Semitones(-1f64))]
    #[case(MtsBytes { note_number: NoteNumber(127), yy: 127, zz: 126 }, Semitones(127.9999f64))]
    #[case(MtsBytes { note_number: NoteNumber(127), yy: 127, zz: 126 }, Semitones(128f64))]
    fn semitones_to_mts_bytes(#[case] expected: MtsBytes, #[case] input: Semitones) {
        assert_eq!(expected, input.to_mts_bytes())
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
    fn frequency_to_mts_bytes(#[case] expected: MtsBytes, #[case] input: Frequency) {
        assert_eq!(expected, input.to_mts_bytes())
    }

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
    fn mts_bytes_to_frequency(#[case] expected: Frequency, #[case] input: MtsBytes) {
        assert_eq!(expected.0, input.to_frequency().0);
    }

    #[rstest]
    #[case("3c0000", MtsBytes { note_number: NoteNumber(60), yy: 0, zz: 0 })]
    #[case("450000", MtsBytes { note_number: NoteNumber(69), yy: 0, zz: 0 })]
    #[case("450a06", MtsBytes { note_number: NoteNumber(69), yy: 10, zz: 6 })]
    #[case("457f06", MtsBytes { note_number: NoteNumber(69), yy: 240, zz: 6 })]
    #[case("7f7f7f", MtsBytes { note_number: NoteNumber(128), yy: 255, zz: 128 })]
    #[case("7f7f7f", MtsBytes { note_number: NoteNumber(127), yy: 127, zz: 127 })]
    fn mts_bytes_to_hex(#[case] expected: &str, #[case] input: MtsBytes) {
        assert_eq!(expected, input.to_hex());
    }

    #[test]
    fn frequency_to_note_number() {
        let (note_number, cent_offset) = Frequency(261.625565f64).to_note_number();
        assert_eq!(60, note_number.0);
        assert!(cent_offset
            .0
            .approx_eq_with_epsilon(0f64, CENT_OFFSET_EPSILON.0));
    }
}
