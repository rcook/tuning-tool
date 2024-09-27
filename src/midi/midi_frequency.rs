use crate::midi::note_number::NoteNumber;
use crate::types::{Cents, Octave};
use crate::u7::U7;
use anyhow::Result;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::Rem;

#[derive(Debug, PartialEq)]
pub(crate) struct MidiFrequency {
    note_number: NoteNumber,
    yy: U7,
    zz: U7,
}

impl MidiFrequency {
    // https://forums.steinberg.net/t/microtonal-midi-messages-vst-3/831268/9
    pub(crate) fn from_note_number(note_number: NoteNumber, delta_cents: Cents) -> Result<Self> {
        let semitones = delta_cents / 100f64;
        let semitones_14bit = (semitones * (0x4000 as f64)) as u16; // i.e. 5406
        let yy = semitones_14bit / 0x80; // i.e. 42
        let zz = semitones_14bit - 0x80 * yy; // i.e. 30
        Self::new(note_number, yy.try_into()?, zz.try_into()?)
    }

    pub(crate) fn from_cents(octave: Octave, cents: Cents) -> Result<Self> {
        let note_number = (octave as usize * 12 + (cents / 100f64) as usize).try_into()?;
        let delta_cents = cents.rem(100f64);
        Self::from_note_number(note_number, delta_cents)
    }

    pub(crate) fn new(note_number: NoteNumber, yy: U7, zz: U7) -> Result<Self> {
        Ok(Self {
            note_number,
            yy,
            zz,
        })
    }

    #[must_use]
    pub(crate) const fn note_number(&self) -> NoteNumber {
        self.note_number
    }

    #[must_use]
    pub(crate) const fn yy(&self) -> U7 {
        self.yy
    }

    #[must_use]
    pub(crate) const fn zz(&self) -> U7 {
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
