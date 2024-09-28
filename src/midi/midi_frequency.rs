use crate::frequency::Frequency;
use crate::midi::note_number::NoteNumber;
use crate::u7::u7;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, PartialEq)]
pub(crate) struct MidiFrequency {
    note_number: NoteNumber,
    yy: u7,
    zz: u7,
}

impl MidiFrequency {
    pub(crate) fn temp(other: Frequency) -> Self {
        let temp0 = other.to_mts_bytes();
        let note_number = NoteNumber::new_lossy(temp0.note_number.0 as u8);
        let yy = temp0.yy.try_into().expect("TBD");
        let zz = temp0.zz.try_into().expect("TBD");
        Self {
            note_number,
            yy,
            zz,
        }
    }

    pub(crate) const fn new(note_number: NoteNumber, yy: u7, zz: u7) -> Self {
        Self {
            note_number,
            yy,
            zz,
        }
    }

    #[must_use]
    pub(crate) const fn note_number(&self) -> NoteNumber {
        self.note_number
    }

    #[must_use]
    pub(crate) const fn yy(&self) -> u7 {
        self.yy
    }

    #[must_use]
    pub(crate) const fn zz(&self) -> u7 {
        self.zz
    }
}

impl Display for MidiFrequency {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{note_number} {yy} {zz}",
            note_number = self.note_number,
            yy = self.yy,
            zz = self.zz
        )
    }
}
