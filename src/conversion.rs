const ROUNDING_SCALE: f64 = 1000000f64;

fn round_to_scale(value: f64) -> f64 {
    (value * ROUNDING_SCALE).round() / ROUNDING_SCALE
}

pub(crate) struct Ratio(f64);

impl Ratio {
    // c.f. valueToCents
    #[allow(unused)]
    #[must_use]
    pub(crate) fn to_cents(&self) -> Cents {
        Cents(1200f64 * self.0.log2())
    }
}

pub(crate) struct Cents(f64);

impl Cents {
    // c.f. centsToValue
    #[allow(unused)]
    #[must_use]
    pub(crate) fn to_ratio(&self) -> Ratio {
        Ratio(2f64.powf(self.0 / 1200f64))
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Frequency(pub(crate) f64);

impl Frequency {
    #[allow(unused)]
    pub(crate) const A4: Self = Self(440f64);

    #[allow(unused)]
    pub(crate) const MIDDLE_C: Self = Self(261.625565f64);

    #[allow(unused)]
    pub(crate) const MIN: Self = Self(8.175798915643707f64);
    #[allow(unused)]
    pub(crate) const MAX: Self = Self(13289.656616f64);

    // c.f. frequencyToCentOffset
    #[allow(unused)]
    #[must_use]
    pub(crate) fn to_cent_offset(&self) -> CentOffset {
        self.to_cent_offset_with_base_frequency(Self::A4)
    }

    #[allow(unused)]
    #[must_use]
    pub(crate) fn to_cent_offset_with_base_frequency(
        &self,
        base_frequency: Frequency,
    ) -> CentOffset {
        CentOffset(Ratio(self.0 / base_frequency.0).to_cents().0)
    }

    // c.f. ftomts
    #[allow(unused)]
    #[must_use]
    pub(crate) fn to_semitones(&self) -> Semitones {
        self.to_semitones_with_ignore_limit(false)
    }

    #[allow(unused)]
    #[must_use]
    pub(crate) fn to_semitones_with_ignore_limit(&self, ignore_limit: bool) -> Semitones {
        if self.0 <= 0f64 {
            return Semitones(0f64);
        }

        if self.0 > Self::MAX.0 && !ignore_limit {
            return Semitones::MAX;
        }

        let semitones = self.semitones_raw().0;
        let value = round_to_scale(semitones);
        Semitones(value)
    }

    // c.f. ftom
    #[allow(unused)]
    #[must_use]
    pub(crate) fn to_note_number(&self) -> (NoteNumber, CentOffset) {
        let semitones = self.semitones_raw();
        let note_number = semitones.0.round();
        let cent_offset = (semitones.0 - note_number) * 100f64;
        (NoteNumber(note_number as i32), CentOffset(cent_offset))
    }

    #[allow(unused)]
    #[must_use]
    pub(crate) fn to_mts_bytes(&self) -> MtsBytes {
        self.to_semitones().to_mts_bytes()
    }

    fn semitones_raw(&self) -> Semitones {
        Semitones((NoteNumber::A4.0 as f64) + 12f64 * (self.0 / Self::A4.0).log2())
    }
}

pub(crate) struct Semitones(f64);

impl Semitones {
    #[allow(unused)]
    pub(crate) const MAX: Self = Self(127.999878f64);

    // c.f. mtsToMtsBytes
    #[allow(unused)]
    #[must_use]
    pub(crate) fn to_mts_bytes(&self) -> MtsBytes {
        if self.0 <= 0f64 {
            return MtsBytes {
                note_number: NoteNumber(0),
                yy: 0,
                zz: 0,
            };
        }

        if self.0 > 127.999878f64 {
            return MtsBytes {
                note_number: NoteNumber(127),
                yy: 127,
                zz: 126,
            };
        }

        let note_number = self.0.trunc();
        let fine = ((0x4000 as f64) * (self.0 - note_number)).round() as u16;

        let yy = ((fine >> 7) & 0x7f) as u8;
        let zz = (fine & 0x7f) as u8;
        MtsBytes {
            note_number: NoteNumber(note_number as i32),
            yy,
            zz,
        }
    }

    #[allow(unused)]
    #[must_use]
    pub(crate) fn to_frequency(&self) -> Frequency {
        let temp: f64 = NoteNumber::A4.0.into();
        Frequency(Frequency::A4.0 * 2f64.powf((self.0 - temp) / 12f64))
    }
}

pub(crate) struct CentOffset(f64);

impl CentOffset {
    // c.f. centOffsetToFrequency
    #[allow(unused)]
    #[must_use]
    pub(crate) fn to_frequency(&self) -> Frequency {
        self.to_frequency_with_base_frequency(Frequency::A4)
    }

    #[allow(unused)]
    #[must_use]
    pub(crate) fn to_frequency_with_base_frequency(&self, base_frequency: Frequency) -> Frequency {
        Frequency(Cents(self.0).to_ratio().0 * base_frequency.0)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct NoteNumber(pub(crate) i32);

impl NoteNumber {
    #[allow(unused)]
    pub(crate) const A4: Self = Self(69);

    // c.f. mtof
    #[allow(unused)]
    #[must_use]
    pub(crate) fn to_frequency(&self) -> Frequency {
        Semitones(self.0.into()).to_frequency()
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct MtsBytes {
    pub(crate) note_number: NoteNumber,
    pub(crate) yy: u8,
    pub(crate) zz: u8,
}

impl MtsBytes {
    // c.f. mtsBytesToMts
    #[allow(unused)]
    #[must_use]
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
    #[allow(unused)]
    #[must_use]
    pub(crate) fn to_frequency(&self) -> Frequency {
        let mts = self.to_semitones();
        println!("mts={:?}", mts.0);
        let frequency = mts.to_frequency();
        Frequency(round_to_scale(frequency.0))
    }

    // c.f. mtsBytesToHex
    #[allow(unused)]
    #[must_use]
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
    use crate::approx_eq::ApproxEq;
    use crate::conversion::{CentOffset, Cents, Frequency, MtsBytes, NoteNumber, Ratio, Semitones};
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
