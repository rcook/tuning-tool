use crate::frequency::Frequency;
use crate::interval::Interval;
use crate::key_mapping::KeyMapping;
use crate::key_mappings::KeyMappings;
use crate::keyboard_mapping::KeyboardMapping;
use crate::midi_note::MidiNote;
use crate::scale::Scale;
use crate::types::KeyNumber;
use anyhow::{anyhow, Result};
use log::trace;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::iter::once;

#[derive(Debug)]
pub(crate) struct KeyFrequencyMapping {
    pub(crate) key: KeyNumber,
    pub(crate) frequency: Frequency,
    pub(crate) degree: usize,
    pub(crate) interval: Interval,
}

impl KeyFrequencyMapping {
    pub(crate) fn compute(
        scale: &Scale,
        keyboard_mapping: &KeyboardMapping,
    ) -> Result<Vec<KeyFrequencyMapping>> {
        let start = keyboard_mapping.start_key().to_u8() as usize;
        let end = keyboard_mapping.end_key().to_u8() as usize;
        Ok(Self::compute_all(
            scale,
            keyboard_mapping.zero_key(),
            keyboard_mapping.reference_key(),
            keyboard_mapping.reference_frequency(),
            keyboard_mapping.key_mappings(),
        )?
        .drain(start..=end)
        .collect::<Vec<_>>())
    }

    fn compute_all(
        scale: &Scale,
        zero_key: &KeyNumber,
        reference_key: &KeyNumber,
        reference_frequency: &Frequency,
        key_mappings: &KeyMappings,
    ) -> Result<Vec<KeyFrequencyMapping>> {
        fn calculate_frequency(
            key: i32,
            keys_per_equave: i32,
            zero_frequency: f64,
            reference: i32,
            reference_ratio: f64,
            equave_ratio: f64,
            interval: &Interval,
        ) -> Frequency {
            let equave = (key - reference).div_euclid(keys_per_equave);
            let ratio = interval.as_ratio().0;
            let (equave, ratio) = if ratio < reference_ratio {
                (equave + 1, ratio)
            } else {
                (equave, ratio)
            };

            Frequency(zero_frequency * ratio * equave_ratio.powi(equave))
        }

        const N: usize = 128;

        let unison = Interval::unison();
        let intervals = Self::select_intervals(scale, &unison, key_mappings)?;
        let zero = zero_key.to_u8() as i32;
        let keys_per_equave = intervals.len();
        let offset = (-zero).rem_euclid(keys_per_equave as i32) as usize;
        let degree_intervals = intervals
            .iter()
            .cycle()
            .skip(offset)
            .take(keys_per_equave)
            .cycle()
            .take(N)
            .collect::<Vec<_>>();

        let reference = reference_key.to_u8() as i32;
        let reference_frequency = reference_frequency.0;
        let reference_ratio = degree_intervals
            .get(reference as usize)
            .expect("Must be in range")
            .1
            .as_ratio()
            .0;

        trace!(
            "Reference key {} at {:.2} Hz",
            reference,
            reference_frequency
        );

        let zero_frequency = reference_frequency / reference_ratio;
        let equave_ratio = scale.equave_ratio().0;

        trace!(
            "Zero key {zero} at {frequency:.2} Hz (unison/prime interval)",
            frequency = calculate_frequency(
                zero,
                keys_per_equave as i32,
                zero_frequency,
                reference,
                reference_ratio,
                equave_ratio,
                &unison,
            )
            .0
        );

        let mut mappings = Vec::with_capacity(N);
        for (i, degree_interval) in degree_intervals.iter().enumerate() {
            let frequency = calculate_frequency(
                i as i32,
                keys_per_equave as i32,
                zero_frequency,
                reference,
                reference_ratio,
                equave_ratio,
                degree_interval.1,
            );

            let mapping = KeyFrequencyMapping {
                key: (i as u8).try_into()?,
                frequency,
                degree: degree_interval.0,
                interval: degree_interval.1.clone(),
            };
            trace!("{mapping}");
            mappings.push(mapping);
        }

        Ok(mappings)
    }

    fn select_intervals<'a>(
        scale: &'a Scale,
        unison: &'a Interval,
        key_mappings: &KeyMappings,
    ) -> Result<Vec<(usize, &'a Interval)>> {
        let interval_count = scale.intervals().len();
        let intervals = once(unison)
            .chain(scale.intervals())
            .take(interval_count)
            .collect::<Vec<_>>();
        let intervals = match key_mappings {
            KeyMappings::Linear => intervals
                .iter()
                .enumerate()
                .map(|(degree, interval)| (degree, *interval))
                .collect::<Vec<_>>(),
            KeyMappings::Custom(key_mappings) => {
                let mut selected_intervals = Vec::new();
                for key_mapping in key_mappings {
                    match key_mapping {
                        KeyMapping::Degree(degree) => {
                            let interval = intervals.get(*degree).ok_or_else(|| {
                                anyhow!("Degree {degree} does not exist in scale")
                            })?;
                            selected_intervals.push((*degree, *interval))
                        }
                        KeyMapping::Unmapped => {
                            todo!("Sparse keyboard mappings not implemented yet!")
                        }
                    }
                }
                selected_intervals
            }
        };

        Ok(intervals)
    }
}

impl Display for KeyFrequencyMapping {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{key:<3}  {name:<4}  {degree:<2}  {interval:<10}  {f:.2} Hz",
            key = self.key.to_u8(),
            name = MidiNote::ALL[self.key.to_u8() as usize].name(),
            degree = self.degree,
            interval = self.interval.to_string(),
            f = self.frequency.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::approx_eq::ApproxEq;
    use crate::frequency::Frequency;
    use crate::key_frequency_mapping::KeyFrequencyMapping;
    use crate::key_mapping::KeyMapping;
    use crate::key_mappings::KeyMappings;
    use crate::keyboard_mapping::KeyboardMapping;
    use crate::scale::Scale;
    use crate::test_util::read_expected_frequencies;
    use crate::types::KeyNumber;
    use anyhow::Result;
    use std::iter::zip;
    use std::path::Path;
    use std::sync::LazyLock;
    use tuning_tool_macros::scale;

    static SCALE_31EDO2: LazyLock<Scale> = LazyLock::new(|| {
        scale![
            38.70968 77.41935 116.12903 154.83871 193.54839
            232.25806 270.96774 309.67742 348.38710 387.09677
            425.80645 464.51613 503.22581 541.93548 580.64516
            619.35484 658.06452 696.77419 735.48387 774.19355
            812.90323 851.61290 890.32258 929.03226 967.74194
            1006.45161 1045.16129 1083.87097 1122.58065 1161.29032
            2/1
        ]
    });

    static BOHLEN_P: LazyLock<Scale> = LazyLock::new(|| {
        scale![
            27/25 25/21 9/7 7/5 75/49 5/3 9/5
            49/25 15/7 7/3 63/25 25/9 3/1
        ]
    });

    static SCALE_24EDO2: LazyLock<Scale> = LazyLock::new(|| {
        scale![
            50.0 100.0 150.0 200.0 250.0 300.0 350.0 400.0 450.0
            500.0 550.0 600.0 650.0 700.0 750.0 800.0 850.0 900.0
            950.0 1000.0 1050.0 1100.0 1150.0 2/1
        ]
    });

    static SCALE_12EDO2: LazyLock<Scale> = LazyLock::new(|| {
        scale![
            100.0 200.0 300.0 400.0 500.0 600.0 700.0 800.0 900.0
            1000.0 1100.0 2/1
        ]
    });

    static CARLOS_SUPER: LazyLock<Scale> = LazyLock::new(|| {
        scale![
            17/16 9/8 6/5 5/4 4/3 11/8 3/2 13/8 5/3 7/4 15/8
            2/1
        ]
    });

    #[test]
    fn scale_31edo2() -> Result<()> {
        check_frequencies(
            "test/31edo2-expected-frequencies.txt",
            &SCALE_31EDO2,
            &KeyboardMapping::new(
                KeyNumber::ZERO,
                KeyNumber::MAX,
                KeyNumber::constant::<69>(),
                KeyNumber::constant::<60>(),
                Frequency(400f64),
                KeyMappings::Custom(vec![
                    KeyMapping::Degree(0),
                    KeyMapping::Degree(3),
                    KeyMapping::Degree(5),
                    KeyMapping::Degree(8),
                    KeyMapping::Degree(10),
                    KeyMapping::Degree(13),
                    KeyMapping::Degree(16),
                    KeyMapping::Degree(18),
                    KeyMapping::Degree(21),
                    KeyMapping::Degree(23),
                    KeyMapping::Degree(26),
                    KeyMapping::Degree(28),
                ]),
            )?,
        )
    }

    #[test]
    fn scale_31edo2_subset() -> Result<()> {
        check_frequencies(
            "test/31edo2-subset-expected-frequencies.txt",
            &SCALE_31EDO2,
            &KeyboardMapping::new(
                KeyNumber::constant::<1>(),
                KeyNumber::constant::<3>(),
                KeyNumber::constant::<69>(),
                KeyNumber::constant::<60>(),
                Frequency(400f64),
                KeyMappings::Custom(vec![
                    KeyMapping::Degree(0),
                    KeyMapping::Degree(3),
                    KeyMapping::Degree(5),
                    KeyMapping::Degree(8),
                    KeyMapping::Degree(10),
                    KeyMapping::Degree(13),
                    KeyMapping::Degree(16),
                    KeyMapping::Degree(18),
                    KeyMapping::Degree(21),
                    KeyMapping::Degree(23),
                    KeyMapping::Degree(26),
                    KeyMapping::Degree(28),
                ]),
            )?,
        )
    }

    #[test]
    fn bohlen_p() -> Result<()> {
        check_frequencies(
            "test/bohlen-p-expected-frequencies.txt",
            &BOHLEN_P,
            &KeyboardMapping::new_full_linear(
                KeyNumber::constant::<69>(),
                KeyNumber::constant::<69>(),
                Frequency::CONCERT_A4,
            )?,
        )?;
        Ok(())
    }

    #[test]
    fn scale_24edo2_432() -> Result<()> {
        check_frequencies(
            "test/24edo2-432-expected-frequencies.txt",
            &SCALE_24EDO2,
            &KeyboardMapping::new_full_linear(
                KeyNumber::constant::<69>(),
                KeyNumber::constant::<69>(),
                Frequency(432f64),
            )?,
        )?;
        Ok(())
    }

    #[test]
    fn scale_12edo2() -> Result<()> {
        check_frequencies(
            "test/12edo2-expected-frequencies.txt",
            &SCALE_12EDO2,
            &KeyboardMapping::new_full_linear(
                KeyNumber::constant::<69>(),
                KeyNumber::constant::<69>(),
                Frequency::CONCERT_A4,
            )?,
        )?;
        Ok(())
    }

    #[test]
    fn carlos_super_zero() -> Result<()> {
        check_frequencies(
            "test/carlos-super-zero-expected-frequencies.txt",
            &CARLOS_SUPER,
            &KeyboardMapping::new_full_linear(KeyNumber::ZERO, KeyNumber::ZERO, Frequency::MIN)?,
        )?;
        Ok(())
    }

    #[test]
    fn carlos_super_69() -> Result<()> {
        check_frequencies(
            "test/carlos-super-69-expected-frequencies.txt",
            &CARLOS_SUPER,
            &KeyboardMapping::new_full_linear(
                KeyNumber::constant::<69>(),
                KeyNumber::constant::<69>(),
                Frequency::CONCERT_A4,
            )?,
        )?;
        Ok(())
    }

    fn check_frequencies<P: AsRef<Path>>(
        expected_frequencies_path: P,
        scale: &Scale,
        keyboard_mapping: &KeyboardMapping,
    ) -> Result<()> {
        let expected_frequencies = read_expected_frequencies(expected_frequencies_path)?;
        let frequencies = KeyFrequencyMapping::compute(scale, keyboard_mapping)?;
        assert_eq!(expected_frequencies.len(), frequencies.len());
        for (expected, actual) in zip(expected_frequencies, frequencies) {
            assert!(actual
                .frequency
                .0
                .approx_eq_with_epsilon(expected, 0.000000001f64))
        }
        Ok(())
    }
}