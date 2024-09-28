use crate::conversion::Frequency;
use crate::conversion::NoteNumber;
use crate::scale::Scale;
use std::iter::zip;
use std::ops::Rem;

pub(crate) type Frequencies = [Frequency; 128];

pub(crate) struct EquaveRatio(pub(crate) f64);

pub(crate) struct Tuning {
    _base_note_number: NoteNumber,
    base_frequency: Frequency,
    equave_ratio: EquaveRatio,
    size: usize,
}

impl Tuning {
    #[allow(unused)]
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

    #[allow(unused)]
    pub(crate) fn get_frequencies(&self, tuning: &Scale) -> Frequencies {
        assert!(tuning.is_octave_repeating());
        let mut reference_frequency = self.base_frequency;
        let mut frequencies = [Frequency(0f64); 128];
        for (i, interval) in zip(0..=127, tuning.intervals().iter().take(self.size).cycle()) {
            if i > 0 && i.rem(self.size) == 0 {
                reference_frequency = Frequency(reference_frequency.0 * self.equave_ratio.0);
            }
            frequencies[i] = Frequency(reference_frequency.0 * interval.to_f64());
        }
        frequencies
    }
}

#[cfg(test)]
mod tests {
    use crate::conversion::Frequency;
    use crate::conversion::NoteNumber;
    use crate::midi::bulk_tuning_dump_reply::BulkTuningDumpReply;
    use crate::midi::midi_frequency::MidiFrequency;
    use crate::resources::RESOURCE_DIR;
    use crate::scala_file::ScalaFile;
    use crate::scale::Scale;
    use crate::tuning::EquaveRatio;
    use crate::tuning::Tuning;
    use crate::u7::{u7, u7_lossy};
    use anyhow::{anyhow, Result};

    #[test]
    fn basics() -> Result<()> {
        let scl_dir = RESOURCE_DIR
            .get_dir("scl")
            .ok_or_else(|| anyhow!("Could not get scl directory"))?;
        let scl_file = scl_dir
            .get_file("scl/carlos_super.scl")
            .ok_or_else(|| anyhow!("Could not get scl file"))?;
        let s = scl_file
            .contents_utf8()
            .ok_or_else(|| anyhow!("Could not convert to string"))?;
        let scala_file = s.parse::<ScalaFile>()?;

        let scale = scala_file.scale();
        assert!(scale.is_octave_repeating());

        let ref_bytes = RESOURCE_DIR
            .get_file("syx/carlos_super.syx")
            .ok_or_else(|| anyhow!("Could not load tuning dump"))?
            .contents()
            .to_vec();

        let frequencies = Tuning::new(NoteNumber(0), Frequency::MIN, EquaveRatio(2f64), 12)
            .get_frequencies(&scale);

        let frequencies = frequencies.map(MidiFrequency::temp);
        let reply =
            BulkTuningDumpReply::new(u7::ZERO, u7_lossy!(8), "carlos_super.mid", frequencies)?;

        let bytes = reply.to_bytes()?;
        assert_eq!(ref_bytes, bytes);
        Ok(())
    }
}
