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
    fn to_mts_bytes(#[case] expected: (u8, u8, u8), #[case] input: f64) {
        let input = Semitones(input);
        let expected = MtsBytes {
            note_number: NoteNumber(expected.0.into()),
            yy: expected.1.into(),
            zz: expected.2.into(),
        };
        assert_eq!(expected, input.to_mts_bytes())
    }
}
