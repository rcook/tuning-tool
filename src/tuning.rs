use crate::frequency::Frequency;
use crate::interval::Interval;
use crate::note_number::NoteNumber;
use crate::scale::Scale;
use num::Zero;
use std::iter::once;
use std::iter::zip;
use std::ops::Rem;

pub(crate) type Frequencies = [Frequency; 128];

pub(crate) struct EquaveRatio(pub(crate) f64);

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

    pub(crate) fn get_frequencies(&self, scale: &Scale) -> Frequencies {
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
    use crate::frequency::Frequency;
    use crate::midi::bulk_tuning_dump_reply::BulkTuningDumpReply;
    use crate::note_number::NoteNumber;
    use crate::resources::RESOURCE_DIR;
    use crate::scala_file::ScalaFile;
    use crate::scale::Scale;
    use crate::tuning::EquaveRatio;
    use crate::tuning::Tuning;
    use crate::u7::{u7, u7_lossy};
    use anyhow::{anyhow, Result};

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

        let frequencies = Tuning::new(NoteNumber(0), Frequency::MIN)
            .get_frequencies(&scale)
            .map(|f| f.to_mts_bytes());
        let reply =
            BulkTuningDumpReply::new(u7::ZERO, u7_lossy!(8), "carlos_super.mid", frequencies)?;

        let bytes = reply.to_bytes()?;
        assert_eq!(ref_bytes, bytes);
        Ok(())
    }
}
