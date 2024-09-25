use crate::semitone::Semitone;

pub(crate) type Frequency = f64;

#[allow(unused)]
pub(crate) fn frequency_to_semitone(frequency: Frequency, base_frequency: Frequency) -> Semitone {
    12f64 * (frequency / base_frequency).log2()
}

#[cfg(test)]
mod tests {
    use crate::frequency::{frequency_to_semitone, Frequency};
    use crate::semitone::Semitone;
    use rstest::rstest;

    fn epsilon_eq(a: f64, b: f64, epsilon: Option<f64>) -> bool {
        (a - b).abs() < epsilon.unwrap_or(f64::EPSILON)
    }

    #[rstest]
    #[case(440f64, 0f64)]
    #[case(880f64, 12f64)]
    #[case(466.1638f64, 1f64)]
    fn basics(#[case] input: Frequency, #[case] expected: Semitone) {
        assert!(epsilon_eq(
            expected,
            frequency_to_semitone(input, 440f64),
            Some(0.00001f64)
        ));
    }
}
