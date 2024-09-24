use crate::frequency::Frequency;
use crate::midi_notes::MidiNotes;
use anyhow::{bail, Error};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;

pub(crate) struct MidiNote(i8);

impl MidiNote {
    pub(crate) fn all() -> MidiNotes {
        MidiNotes::all()
    }

    pub(crate) fn nearest_below(frequency: Frequency) -> (MidiNote, Frequency) {
        Self::nearest_below_with_reference(frequency, Frequency::concert_a())
    }

    pub(crate) fn nearest_below_with_reference(
        frequency: Frequency,
        reference: Frequency,
    ) -> (MidiNote, Frequency) {
        let frequency = frequency.to_f64();
        let value = (12f64 * (frequency / reference.to_f64()).log2() + 69f64) as i8;
        let midi_note = Self(value);
        let rem = (frequency - midi_note.frequency_with_reference(reference).to_f64()).into();
        (midi_note, rem)
    }

    pub(crate) fn frequency(&self) -> Frequency {
        self.frequency_with_reference(Frequency::concert_a())
    }

    pub(crate) fn frequency_with_reference(&self, reference: Frequency) -> Frequency {
        Into::<Frequency>::into(
            (reference.to_f64() / 32f64) * 2f64.powf((self.0 as f64 - 9f64) / 12f64),
        )
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
