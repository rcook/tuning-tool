use crate::frequency::Frequency;
use crate::semitones::Semitones;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct NoteNumber(pub(crate) i32);

impl NoteNumber {
    pub(crate) const A4: Self = Self(69);

    // c.f. mtof
    pub(crate) fn to_frequency(&self) -> Frequency {
        Semitones(self.0.into()).to_frequency()
    }
}

#[cfg(test)]
mod tests {
    use crate::approx_eq::ApproxEq;
    use crate::note_number::NoteNumber;

    #[test]
    fn to_frequency() {
        assert!(NoteNumber(60)
            .to_frequency()
            .0
            .approx_eq_with_epsilon(261.625565, 0.000001f64));
    }
}
