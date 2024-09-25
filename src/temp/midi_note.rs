use crate::{frequency::Frequency, note_number::NoteNumber};
use std::fmt::{Display, Formatter, Result as FmtResult};

include!(concat!(env!("OUT_DIR"), "/midi_note_frequencies_consts.rs"));

#[derive(Clone, Copy, Debug)]
pub(crate) struct MidiNote {
    note_number: NoteNumber,
    frequency: Frequency,
    name: &'static str,
}

impl MidiNote {
    pub(crate) const ALL: [Self; 128] = {
        const UNINITIALIZED: MidiNote = MidiNote::new(-1, 0f64, "");
        let mut values = [UNINITIALIZED; 128];
        let mut i = 0;
        while i >= 0 {
            let index = i as usize;
            values[index] = MidiNote::new(i, MIDI_NOTES[index].0, MIDI_NOTES[index].1);
            i = i.wrapping_add(1);
        }
        values
    };

    #[allow(unused)]
    pub(crate) fn nearest_below_or_equal(frequency: Frequency) -> MidiNote {
        let mut i = 127;
        while i > 0 && MIDI_NOTES[i].0 > frequency {
            i -= 1;
        }
        Self::ALL[i]
    }

    #[must_use]
    pub(crate) const fn note_number(&self) -> NoteNumber {
        self.note_number
    }

    #[must_use]
    pub(crate) const fn name(&self) -> &str {
        &self.name
    }

    const fn new(note_number: NoteNumber, frequency: Frequency, name: &'static str) -> Self {
        Self {
            note_number,
            frequency,
            name,
        }
    }
}

impl Display for MidiNote {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} ({} Hz)", self.note_number, self.frequency)
    }
}
