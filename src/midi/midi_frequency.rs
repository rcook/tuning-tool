use crate::midi::midi_note_number::MidiNoteNumber;
use anyhow::{bail, Result};

#[derive(Debug)]
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
    pub(crate) fn compute(note_number: MidiNoteNumber, delta_cents: f64) -> Result<Self> {
        let xx = note_number;
        let semitones = delta_cents / 100f64;
        let semitones_14bit = (semitones * (0x4000 as f64)) as u16; // i.e. 5406
        let yy = semitones_14bit / 0x80; // i.e. 42
        let zz = semitones_14bit - 0x80 * yy; // i.e. 30
        Self::new(xx as u8, yy as u8, zz as u8)
    }

    pub(crate) fn new(xx: u8, yy: u8, zz: u8) -> Result<Self> {
        if xx > 127 || yy > 127 || zz > 127 {
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
