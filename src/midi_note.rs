use crate::hertz::Hertz;
use crate::midi_notes::MidiNotes;
use anyhow::{bail, Error};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;

pub(crate) const CONCERT_A: Hertz = Hertz::new(440f64);

pub(crate) struct MidiNote(i8);

impl MidiNote {
    pub(crate) fn all() -> MidiNotes {
        MidiNotes::all()
    }

    pub(crate) fn nearest_below(frequency: Hertz) -> (MidiNote, Hertz) {
        Self::nearest_below_with_reference(frequency, CONCERT_A)
    }

    pub(crate) fn nearest_below_with_reference(
        frequency: Hertz,
        reference: Hertz,
    ) -> (MidiNote, Hertz) {
        let frequency = frequency.to_f64();
        let value = (12f64 * (frequency / reference.to_f64()).log2() + 69f64) as i8;
        let midi_note = Self(value);
        let rem = (frequency - midi_note.to_hertz_with_reference(reference).to_f64())
            .try_into()
            .expect("TBD");
        (midi_note, rem)
    }

    pub(crate) fn to_hertz(&self) -> Hertz {
        self.to_hertz_with_reference(CONCERT_A)
    }
    pub(crate) fn to_hertz_with_reference(&self, reference: Hertz) -> Hertz {
        Hertz::new((reference.to_f64() / 32f64) * 2f64.powf((self.0 as f64 - 9f64) / 12f64))
    }
}

impl Display for MidiNote {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "[MIDI {value}]", value = self.0)
    }
}

impl TryFrom<i8> for MidiNote {
    type Error = Error;

    fn try_from(value: i8) -> StdResult<Self, Self::Error> {
        if value < 0 {
            bail!("Invalid MIDI note value {value}")
        }

        Ok(Self(value))
    }
}
