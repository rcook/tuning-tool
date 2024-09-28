use crate::conversion::Frequency;
use crate::conversion::NoteNumber;
use crate::scala::scale_note::ScaleNote;
use crate::scala::tuning::Tuning;
use num::{One, ToPrimitive};
use std::iter::zip;

pub(crate) type Frequencies = [Frequency; 128];

pub(crate) struct EquaveRatio(pub(crate) f64);

pub(crate) struct Scale {
    _base_note_number: NoteNumber,
    base_frequency: Frequency,
    equave_ratio: EquaveRatio,
    size: usize,
}

impl Scale {
    pub(crate) fn new(
        base_note_number: NoteNumber,
        base_frequency: Frequency,
        equave_ratio: EquaveRatio,
        size: usize,
    ) -> Self {
        Self {
            _base_note_number: base_note_number,
            base_frequency,
            equave_ratio,
            size,
        }
    }

    pub(crate) fn get_frequencies(&self, tuning: &Tuning) -> Frequencies {
        let mut reference_frequency = self.base_frequency;
        let mut frequencies = [Frequency(0f64); 128];
        for (i, scale_note) in zip(0..=127, tuning.notes().take(self.size).cycle()) {
            let ratio = match &scale_note {
                &ScaleNote::Ratio(ratio) => ratio,
                _ => todo!(),
            };
            if i > 0 && ratio.is_one() {
                reference_frequency = Frequency(reference_frequency.0 * self.equave_ratio.0);
            }
            frequencies[i] = Frequency(reference_frequency.0 * ratio.to_f64().expect("TBD"));
        }
        frequencies
    }
}
