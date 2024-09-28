use crate::frequency::Frequency;
use crate::semitones::Semitones;

#[derive(Debug, PartialEq)]
pub(crate) struct NoteNumber(pub(crate) i32);

impl NoteNumber {
    pub(crate) const A4: Self = Self(69);

    // c.f. mtof
    pub(crate) fn to_frequency(&self) -> Frequency {
        Semitones(self.0.into()).to_frequency()
    }
}
