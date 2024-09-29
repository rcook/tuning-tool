use crate::frequency::Frequency;
use crate::interval::Interval;
use crate::note_number::NoteNumber;
use crate::scale::Scale;
use num::Zero;
use std::iter::once;
use std::iter::zip;
use std::ops::Rem;

pub(crate) type Frequencies = [Frequency; 128];

pub(crate) struct Tuning {
    _base_note_number: NoteNumber,
    base_frequency: Frequency,
}

impl Tuning {
    pub(crate) fn new(base_note_number: NoteNumber, base_frequency: Frequency) -> Self {
        Self {
            _base_note_number: base_note_number,
            base_frequency,
        }
    }

    pub(crate) fn calculate_frequencies(&self, scale: &Scale) -> Frequencies {
        let equave_ratio = scale.equave_ratio();
        assert_eq!(2f64, equave_ratio.0); // TBD: Haven't tested with anything other than octave-repeating scales!
        let scale_size = scale.intervals().len();
        let unison = Interval::unison();
        let intervals = once(&unison).chain(scale.intervals().iter().take(scale_size - 1));

        let mut reference_frequency = self.base_frequency;
        let mut frequencies = [Frequency(0f64); 128];
        for (i, interval) in zip(0..=127, intervals.cycle()) {
            if i > 0 && i.rem(scale_size).is_zero() {
                reference_frequency = Frequency(reference_frequency.0 * equave_ratio.0);
            }
            frequencies[i] = Frequency(reference_frequency.0 * interval.to_f64());
        }
        frequencies
    }
}

#[cfg(test)]
mod tests {
    use crate::bulk_dump_reply::BulkDumpReply;
    use crate::consts::U7_ZERO;
    use crate::frequency::Frequency;
    use crate::note_number::NoteNumber;
    use crate::resources::RESOURCE_DIR;
    use crate::scala_file::ScalaFile;
    use crate::tuning::Tuning;
    use anyhow::{anyhow, Result};
    use midly::num::u7;

    #[test]
    fn basics() -> Result<()> {
        let ref_bytes = RESOURCE_DIR
            .get_file("syx/carlos_super.syx")
            .ok_or_else(|| anyhow!("Could not load tuning dump"))?
            .contents()
            .to_vec();
        let scl_file = RESOURCE_DIR
            .get_file("scl/carlos_super.scl")
            .ok_or_else(|| anyhow!("Could not get scl file"))?;
        let s = scl_file
            .contents_utf8()
            .ok_or_else(|| anyhow!("Could not convert to string"))?;
        let scala_file = s.parse::<ScalaFile>()?;

        let scale = scala_file.scale();

        let frequencies = Tuning::new(NoteNumber::ZERO, Frequency::MIN)
            .calculate_frequencies(scale)
            .map(|f| f.to_mts_entry());
        let reply = BulkDumpReply::new(
            U7_ZERO,
            u7::from_int_lossy(8),
            "carlos_super.mid".parse()?,
            frequencies,
        )?;

        let bytes = reply.to_bytes_with_start_and_end()?;
        assert_eq!(ref_bytes, bytes);
        Ok(())
    }
}
