use crate::frequency::Frequency;
use crate::mts_bytes::MtsBytes;
use crate::note_number::NoteNumber;

pub(crate) struct Semitones(pub(crate) f64);

impl Semitones {
    pub(crate) const MAX: Self = Self(127.999878f64);

    // c.f. mtsToMtsBytes
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

    pub(crate) fn to_frequency(&self) -> Frequency {
        let temp: f64 = NoteNumber::A4.0.into();
        Frequency(Frequency::A4.0 * 2f64.powf((self.0 - temp) / 12f64))
    }
}

#[cfg(test)]
mod tests {
    use crate::mts_bytes::MtsBytes;
    use crate::note_number::NoteNumber;
    use crate::semitones::Semitones;
    use rstest::rstest;

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
    fn to_mts_bytes(#[case] expected: MtsBytes, #[case] input: Semitones) {
        assert_eq!(expected, input.to_mts_bytes())
    }
}
