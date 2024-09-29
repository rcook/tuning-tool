use crate::note_number::NoteNumber;
use crate::types::Frequency;
use std::fmt::{Display, Formatter, Result as FmtResult};

include!(concat!(env!("OUT_DIR"), "/midi_note_generated.rs"));

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct MidiNote {
    note_number: NoteNumber,
    name: &'static str,
    frequency: Frequency,
}

impl MidiNote {
    pub(crate) const ALL: [MidiNote; 128] = ALL_MIDI_NOTES;

    pub(crate) fn nearest_below_or_equal(frequency: Frequency) -> Option<(MidiNote, Frequency)> {
        let mut i = 127;
        while i > 0 && ALL_MIDI_NOTES[i].frequency() > frequency {
            i -= 1;
        }

        let midi_note = ALL_MIDI_NOTES[i];
        let delta = frequency - midi_note.frequency();
        if delta >= 0f64 {
            Some((midi_note, delta))
        } else {
            None
        }
    }

    pub(crate) const fn note_number(&self) -> NoteNumber {
        self.note_number
    }

    pub(crate) const fn name(&self) -> &str {
        self.name
    }

    pub(crate) const fn frequency(&self) -> Frequency {
        self.frequency
    }

    const fn new(note_number: NoteNumber, name: &'static str, frequency: Frequency) -> Self {
        Self {
            note_number,
            name,
            frequency,
        }
    }
}

impl Display for MidiNote {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} ({} Hz)", self.note_number.0, self.frequency)
    }
}

#[cfg(test)]
mod tests {
    use crate::approx_eq::ApproxEq;
    use crate::midi_note::MidiNote;
    use crate::note_number::NoteNumber;
    use anyhow::Result;

    #[test]
    fn basics() {
        assert_eq!(128, MidiNote::ALL.len());
    }

    #[test]
    fn concert_a() {
        let (midi_note, rem) = MidiNote::nearest_below_or_equal(450f64).expect("Must succeed");
        assert!(rem.approx_eq_with_epsilon(10f64, 0.01));

        assert_eq!(69, midi_note.note_number.0);
        assert_eq!("A4", midi_note.name());
        assert!(midi_note.frequency().approx_eq_with_epsilon(440f64, 0.001));

        let (other_midi_note, _) = MidiNote::nearest_below_or_equal(450f64).expect("Must succeed");
        assert_eq!(midi_note, other_midi_note);
    }

    #[test]
    fn limits() {
        assert!(MidiNote::nearest_below_or_equal(0f64).is_none());

        let (midi_note, rem) = MidiNote::nearest_below_or_equal(8.2f64).expect("Must succeed");
        assert!(rem.approx_eq_with_epsilon(0.03f64, 0.01));
        assert_eq!(MidiNote::ALL[0], midi_note);

        let (midi_note, rem) = MidiNote::nearest_below_or_equal(80000f64).expect("Must succeed");
        assert!(rem.approx_eq_with_epsilon(67456.15f64, 0.01));
        assert_eq!(MidiNote::ALL[127], midi_note);
    }

    #[test]
    fn frequencies() -> Result<()> {
        for i in 0..=127 {
            let note_number = NoteNumber::new_lossy(i as u8);
            let frequency = 440f64 * 2f64.powf((i as i32 - 69) as f64 / 12f64);
            let midi_note = MidiNote::ALL[i];
            assert_eq!(note_number, midi_note.note_number());

            // This must be an exact match
            assert_eq!(frequency, midi_note.frequency());
        }
        Ok(())
    }
}
