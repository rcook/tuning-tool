use crate::consts::{U7_MAX, U7_ZERO};
use crate::frequency::Frequency;
use crate::semitones::Semitones;
use midly::num::u7;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct NoteNumber(pub(crate) u7);

impl NoteNumber {
    pub(crate) const ZERO: Self = Self(U7_ZERO);
    pub(crate) const A4: Self = Self::new_lossy(69);
    pub(crate) const MAX: Self = Self(U7_MAX);

    pub(crate) const fn new_lossy(value: u8) -> Self {
        Self(u7::from_int_lossy(value))
    }

    // c.f. mtof
    #[allow(unused)]
    pub(crate) fn to_frequency(&self) -> Frequency {
        Semitones(self.0.as_int() as f64).to_frequency()
    }
}

#[cfg(test)]
mod tests {
    use crate::approx_eq::ApproxEq;
    use crate::note_number::NoteNumber;

    #[test]
    fn to_frequency() {
        assert!(NoteNumber::new_lossy(60)
            .to_frequency()
            .0
            .approx_eq_with_epsilon(261.625565, 0.000001f64));
    }
}
