use crate::frequency::Frequency;
use crate::note_number::NoteNumber;
use std::fmt::{Display, Formatter, Result as FmtResult};

include!(concat!(env!("OUT_DIR"), "/midi_note_consts.rs"));

#[derive(Clone, Copy, Debug)]
pub(crate) struct MidiNote {
    note_number: NoteNumber,
    name: &'static str,
    frequency: Frequency,
}

impl MidiNote {
    pub(crate) const ALL: [MidiNote; 128] = ALL_MIDI_NOTES;

    pub(crate) fn nearest_below_or_equal(frequency: Frequency) -> (MidiNote, Frequency) {
        let mut i = 127;
        while i > 0 && ALL_MIDI_NOTES[i].frequency() > frequency {
            i -= 1;
        }

        let midi_note = ALL_MIDI_NOTES[i];
        let delta = frequency - midi_note.frequency();
        (midi_note, delta)
    }

    #[must_use]
    pub(crate) const fn note_number(&self) -> NoteNumber {
        self.note_number
    }

    #[must_use]
    pub(crate) const fn name(&self) -> &str {
        self.name
    }

    #[must_use]
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
        write!(f, "{} ({} Hz)", self.note_number, self.frequency)
    }
}
