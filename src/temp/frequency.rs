use crate::temp::semitone::Semitone;

pub(crate) type Frequency = f64;

#[allow(unused)]
pub(crate) fn frequency_to_semitone(frequency: Frequency, base_frequency: Frequency) -> Semitone {
    12f64 * (frequency / base_frequency).log2()
}

#[cfg(test)]
mod tests {
    use crate::num::ApproxEq;
    use crate::temp::frequency::{frequency_to_semitone, Frequency};
    use crate::temp::semitone::Semitone;
    use rstest::rstest;

    #[rstest]
    #[case(440f64, 0f64)]
    #[case(880f64, 12f64)]
    #[case(466.1638f64, 1f64)]
    fn basics(#[case] input: Frequency, #[case] expected: Semitone) {
        assert!(frequency_to_semitone(input, 440f64).approx_eq_with_epsilon(expected, 0.00001f64));
    }
}
