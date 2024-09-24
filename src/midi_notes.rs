use crate::midi_note::MidiNote;

pub(crate) struct MidiNotes(i8);

impl MidiNotes {
    pub(crate) fn all() -> Self {
        Self(-1)
    }
}

impl Iterator for MidiNotes {
    type Item = MidiNote;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 = self.0.wrapping_add(1);
        if self.0 >= 0 {
            Some(self.0.try_into().expect("Guaranteed to be in range"))
        } else {
            None
        }
    }
}
