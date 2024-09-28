use crate::cent_offset::CentOffset;
use crate::mts_bytes::MtsBytes;
use crate::note_number::NoteNumber;
use crate::num::round_default_scale;
use crate::ratio::Ratio;
use crate::semitones::Semitones;

#[derive(Clone, Copy)]
pub(crate) struct Frequency(pub(crate) f64);

impl Frequency {
    pub(crate) const A4: Self = Self(440f64);
    pub(crate) const MIDDLE_C: Self = Self(261.625565f64);
    pub(crate) const MIN: Self = Self(8.175798915643707f64);
    pub(crate) const MAX: Self = Self(13289.656616f64);

    // c.f. frequencyToCentOffset
    pub(crate) fn to_cent_offset(&self) -> CentOffset {
        self.to_cent_offset_with_base_frequency(Self::A4)
    }

    pub(crate) fn to_cent_offset_with_base_frequency(
        &self,
        base_frequency: Frequency,
    ) -> CentOffset {
        CentOffset(Ratio(self.0 / base_frequency.0).to_cents().0)
    }

    // c.f. ftomts
    pub(crate) fn to_semitones(&self) -> Semitones {
        self.to_semitones_with_ignore_limit(false)
    }

    pub(crate) fn to_semitones_with_ignore_limit(&self, ignore_limit: bool) -> Semitones {
        if self.0 <= 0f64 {
            return Semitones(0f64);
        }

        if self.0 > Self::MAX.0 && !ignore_limit {
            return Semitones::MAX;
        }

        let semitones = self.semitones_raw().0;
        let value = round_default_scale(semitones);
        Semitones(value)
    }

    // c.f. ftom
    pub(crate) fn to_note_number(&self) -> (NoteNumber, CentOffset) {
        let semitones = self.semitones_raw();
        let note_number = semitones.0.round();
        let cent_offset = (semitones.0 - note_number) * 100f64;
        (NoteNumber(note_number as i32), CentOffset(cent_offset))
    }

    pub(crate) fn to_mts_bytes(&self) -> MtsBytes {
        self.to_semitones().to_mts_bytes()
    }

    fn semitones_raw(&self) -> Semitones {
        Semitones((NoteNumber::A4.0 as f64) + 12f64 * (self.0 / Self::A4.0).log2())
    }
}
