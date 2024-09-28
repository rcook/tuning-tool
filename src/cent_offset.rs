use crate::cents::Cents;
use crate::frequency::Frequency;

pub(crate) struct CentOffset(pub(crate) f64);

impl CentOffset {
    // c.f. centOffsetToFrequency
    pub(crate) fn to_frequency(&self) -> Frequency {
        self.to_frequency_with_base_frequency(Frequency::A4)
    }

    pub(crate) fn to_frequency_with_base_frequency(&self, base_frequency: Frequency) -> Frequency {
        Frequency(Cents(self.0).to_ratio().0 * base_frequency.0)
    }
}
