use crate::{coerce::really_really_unsafe_coerce_slice, midi_value::MidiValue};
use anyhow::{bail, Result};
use tuning_tool_lib::U7;

pub(crate) struct MidiMessageBuilder {
    required_len: usize,
    values: Vec<MidiValue>,
}

impl MidiMessageBuilder {
    pub(crate) fn with_required_len(required_len: usize) -> Self {
        Self {
            required_len,
            values: Vec::with_capacity(required_len),
        }
    }

    pub(crate) fn push<U: U7>(&mut self, value: U) -> U {
        self.values.push(MidiValue::from_u8_lossy(value.to_u8()));
        value
    }

    pub(crate) fn extend_from_slice<U: U7>(&mut self, other: &[U]) {
        let other = really_really_unsafe_coerce_slice(other);
        self.values.extend_from_slice(other)
    }

    pub(crate) fn finalize(self) -> Result<Vec<MidiValue>> {
        if self.values.len() != self.required_len {
            bail!("MIDI value vector was not expected length")
        }
        Ok(self.values)
    }
}
