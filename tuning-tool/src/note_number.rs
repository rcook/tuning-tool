use crate::frequency::Frequency;
use crate::semitones::Semitones;
use crate::types::u7_newtype;

u7_newtype!(NoteNumber, pub(crate));

impl NoteNumber {
    pub(crate) const A4: Self = Self::constant::<69>();

    // c.f. mtof
    #[allow(unused)]
    pub(crate) fn to_frequency(&self) -> Frequency {
        Semitones(self.to_u8() as f64).to_frequency()
    }
}

#[cfg(test)]
mod tests {
    use crate::approx_eq::ApproxEq;
    use crate::midi_note::MidiNote;
    use crate::note_number::NoteNumber;
    use anyhow::Result;

    #[test]
    fn to_frequency() {
        assert!(NoteNumber::constant::<60>()
            .to_frequency()
            .0
            .approx_eq_with_epsilon(261.625565, 0.000001f64));
    }

    #[test]
    fn all_note_numbers() -> Result<()> {
        for i in 0..=127 {
            let note_number = NoteNumber::try_from(i)?;
            let frequency = note_number.to_frequency();
            let hardcoded_frequency = MidiNote::ALL[i as usize].frequency();
            assert_eq!(hardcoded_frequency, frequency);
        }
        Ok(())
    }
}
