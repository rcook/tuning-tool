use crate::midi::midi_note_number::MidiNoteNumber;
use crate::num::is_u7;
use crate::types::{Cents, Octave};
use anyhow::{bail, Result};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::Rem;

#[derive(Debug, PartialEq)]
pub(crate) struct MidiFrequency {
    #[allow(unused)]
    xx: u8,

    #[allow(unused)]
    yy: u8,

    #[allow(unused)]
    zz: u8,
}

impl MidiFrequency {
    // https://forums.steinberg.net/t/microtonal-midi-messages-vst-3/831268/9
    pub(crate) fn from_note_number(
        note_number: MidiNoteNumber,
        delta_cents: Cents,
    ) -> Result<Self> {
        let xx = note_number;
        let semitones = delta_cents / 100f64;
        let semitones_14bit = (semitones * (0x4000 as f64)) as u16; // i.e. 5406
        let yy = semitones_14bit / 0x80; // i.e. 42
        let zz = semitones_14bit - 0x80 * yy; // i.e. 30
        Self::new(xx as u8, yy as u8, zz as u8)
    }

    pub(crate) fn from_cents(octave: Octave, cents: Cents) -> Result<Self> {
        let note_number = (octave as usize * 12 + (cents / 100f64) as usize) as MidiNoteNumber;
        let delta_cents = cents.rem(100f64);
        Self::from_note_number(note_number, delta_cents)
    }

    pub(crate) fn new(xx: u8, yy: u8, zz: u8) -> Result<Self> {
        if !is_u7(xx) || !is_u7(yy) || !is_u7(zz) {
            bail!("Invalid values for xx, yy, zz")
        }

        Ok(Self { xx, yy, zz })
    }

    #[must_use]
    pub(crate) const fn xx(&self) -> u8 {
        self.xx
    }

    #[must_use]
    pub(crate) const fn yy(&self) -> u8 {
        self.yy
    }

    #[must_use]
    pub(crate) const fn zz(&self) -> u8 {
        self.zz
    }
}

impl Display for MidiFrequency {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{xx:02X} {yy:02X} {zz:02X}",
            xx = self.xx,
            yy = self.yy,
            zz = self.zz
        )
    }
}
