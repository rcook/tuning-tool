use crate::cents::Cents;
use crate::frequency::Frequency;
use crate::types::f64_newtype;

f64_newtype!(CentOffset, pub(crate));

impl CentOffset {
    // c.f. centOffsetToFrequency
    #[allow(unused)]
    pub(crate) fn to_frequency(&self) -> Frequency {
        self.to_frequency_with_base_frequency(Frequency::A4)
    }

    #[allow(unused)]
    pub(crate) fn to_frequency_with_base_frequency(&self, base_frequency: Frequency) -> Frequency {
        Frequency(Cents(self.0).to_ratio().0 * base_frequency.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::approx_eq::ApproxEq;
    use crate::cent_offset::CentOffset;
    use crate::frequency::Frequency;

    #[test]
    fn basics() {
        assert!(CentOffset(1200f64).to_frequency().0.approx_eq(880f64));
        assert!(CentOffset(1200f64)
            .to_frequency_with_base_frequency(Frequency(220f64))
            .0
            .approx_eq(440f64));
    }
}
