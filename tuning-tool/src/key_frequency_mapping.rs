// Copyright (c) 2024 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

use crate::frequency::Frequency;
use crate::interval::Interval;
use crate::key_mapping::KeyMapping;
use crate::key_mappings::KeyMappings;
use crate::keyboard_mapping::KeyboardMapping;
use crate::midi_note::MidiNote;
use crate::scale::Scale;
use crate::symbolic::evaluate;
use crate::types::KeyNumber;
use anyhow::{anyhow, bail, Result};
use log::trace;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::iter::once;
use tuning_tool_lib::symbolic::Expression;

#[derive(Debug)]
pub(crate) struct KeyFrequencyMapping {
    pub(crate) key: KeyNumber,
    pub(crate) frequency: Expression,
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
        .flatten()
        .collect())
    }

    fn compute_all(
        scale: &Scale,
        zero_key: &KeyNumber,
        reference_key: &KeyNumber,
        reference_frequency: &Frequency,
        key_mappings: &KeyMappings,
    ) -> Result<Vec<Option<KeyFrequencyMapping>>> {
        fn calculate_frequency(
            key: i32,
            keys_per_equave: i32,
            zero_frequency: Expression,
            reference: i32,
            reference_ratio: Expression,
            equave_ratio: Expression,
            interval: &Interval,
        ) -> Expression {
            let equave = (key - reference).div_euclid(keys_per_equave);
            let ratio = interval.as_ratio();
            let equave = if evaluate(ratio.clone()) < evaluate(reference_ratio) {
                equave + 1
            } else {
                equave
            };

            zero_frequency * ratio * equave_ratio.pow(Expression::new_z(equave))
        }

        const N: usize = 128;

        let unison = Interval::unison();
        let intervals = IntervalInfo::select(scale, &unison, key_mappings)?;
        let zero = zero_key.to_u8() as i32;
        let keys_per_equave = intervals.len();
        let offset = (-zero).rem_euclid(keys_per_equave as i32) as usize;
        let intervals = intervals
            .iter()
            .cycle()
            .skip(offset)
            .take(keys_per_equave)
            .cycle()
            .take(N)
            .collect::<Vec<_>>();

        let reference = reference_key.to_u8() as i32;
        let reference_frequency = reference_frequency.0;

        let IntervalInfo::Mapping { interval, .. } =
            intervals.get(reference as usize).expect("Must be in range")
        else {
            bail!("Reference key is not in mapping");
        };

        let reference_ratio = interval.as_ratio();

        trace!(
            "Reference key {} at {:.2} Hz",
            reference,
            reference_frequency
        );

        let zero_frequency = Expression::new_r(reference_frequency) / reference_ratio.clone();
        let equave_ratio = scale.equave_ratio();

        trace!(
            "Zero key {zero} at {frequency:.2} Hz (unison/prime interval)",
            frequency = calculate_frequency(
                zero,
                keys_per_equave as i32,
                zero_frequency.clone(),
                reference,
                reference_ratio.clone(),
                equave_ratio.clone(),
                &unison,
            )
        );

        intervals
            .iter()
            .enumerate()
            .map(|(i, interval_info)| {
                Ok(match interval_info {
                    IntervalInfo::Mapping { degree, interval } => {
                        let frequency = calculate_frequency(
                            i as i32,
                            keys_per_equave as i32,
                            zero_frequency.clone(),
                            reference,
                            reference_ratio.clone(),
                            equave_ratio.clone(),
                            interval,
                        );
                        let mapping = KeyFrequencyMapping {
                            key: (i as u8).try_into()?,
                            frequency,
                            degree: *degree,
                            interval: (*interval).clone(),
                        };
                        trace!("{mapping}");
                        Some(mapping)
                    }
                    IntervalInfo::Unmapped => {
                        trace!("key {i} is unmapped");
                        None
                    }
                })
            })
            .collect::<Result<Vec<_>>>()
    }
}

impl Display for KeyFrequencyMapping {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{key:<3}  {name:<4}  {degree:<2}  {interval:<12}  {f:>9.2} Hz  {symbolic}",
            key = self.key.to_u8(),
            name = MidiNote::ALL[self.key.to_u8() as usize].name(),
            degree = self.degree,
            interval = self.interval.to_string(),
            f = evaluate(self.frequency.clone()),
            symbolic = self.frequency,
        )
    }
}

enum IntervalInfo<'a> {
    Mapping {
        degree: usize,
        interval: &'a Interval,
    },
    Unmapped,
}

impl<'a> IntervalInfo<'a> {
    fn select(
        scale: &'a Scale,
        unison: &'a Interval,
        key_mappings: &KeyMappings,
    ) -> Result<Vec<Self>> {
        let interval_count = scale.intervals().len();
        let intervals = once(unison)
            .chain(scale.intervals())
            .take(interval_count)
            .collect::<Vec<_>>();
        Ok(match key_mappings {
            KeyMappings::Linear => intervals
                .iter()
                .enumerate()
                .map(|(degree, interval)| Self::Mapping { degree, interval })
                .collect::<Vec<_>>(),
            KeyMappings::Custom(key_mappings) => key_mappings
                .iter()
                .map(|key_mapping| {
                    Ok(match key_mapping {
                        KeyMapping::Degree(degree) => {
                            let interval = intervals.get(*degree).ok_or_else(|| {
                                anyhow!("Degree {degree} does not exist in scale")
                            })?;
                            Self::Mapping {
                                degree: *degree,
                                interval,
                            }
                        }
                        KeyMapping::Unmapped => Self::Unmapped,
                    })
                })
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::approx_eq::ApproxEq;
    use crate::frequency::Frequency;
    use crate::kbm_file::KbmFile;
    use crate::key_frequency_mapping::KeyFrequencyMapping;
    use crate::key_mapping::KeyMapping;
    use crate::key_mappings::KeyMappings;
    use crate::keyboard_mapping::KeyboardMapping;
    use crate::resources::RESOURCE_DIR;
    use crate::scale::Scale;
    use crate::scl_file::SclFile;
    use crate::symbolic::evaluate;
    use crate::test_util::{read_expected_frequencies, read_scala_tuning_dump};
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
    fn sparse_mapping() -> Result<()> {
        let expected_frequencies =
            read_scala_tuning_dump("test/22edo2-scala-frequencies.txt", true)
                .iter()
                .map(|(_, f)| f.0)
                .collect::<Vec<_>>();

        let scl_file = RESOURCE_DIR
            .get_file("test/22edo2.scl")
            .expect("Must exist")
            .contents_utf8()
            .expect("Must contain UTF-8")
            .parse::<SclFile>()?;

        let kbm_file = RESOURCE_DIR
            .get_file("test/22edo2.kbm")
            .expect("Must exist")
            .contents_utf8()
            .expect("Must contain UTF-8")
            .parse::<KbmFile>()?;

        let frequencies =
            KeyFrequencyMapping::compute(scl_file.scale(), kbm_file.keyboard_mapping())?;
        for (expected, actual) in zip(expected_frequencies, frequencies) {
            assert!(evaluate(actual.frequency).approx_eq_with_epsilon(expected, 0.0001f64))
        }
        Ok(())
    }

    #[test]
    fn scale_31edo2_69() -> Result<()> {
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
    fn scale_31edo2_57() -> Result<()> {
        check_frequencies(
            "test/31edo2-expected-frequencies.txt",
            &SCALE_31EDO2,
            &KeyboardMapping::new(
                KeyNumber::ZERO,
                KeyNumber::MAX,
                KeyNumber::constant::<57>(),
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

    #[test]
    fn scale_31edo2_69_scala_tuning_dump() -> Result<()> {
        let mappings = read_scala_tuning_dump("test/scala-frequencies.txt", true);
        let expected_frequencies = mappings.iter().map(|(_, f)| f).collect::<Vec<_>>();
        let scale = &*SCALE_31EDO2;
        let keyboard_mapping = KeyboardMapping::new(
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
        )?;

        let frequencies = KeyFrequencyMapping::compute(scale, &keyboard_mapping)?;
        assert_eq!(expected_frequencies.len(), frequencies.len());
        for (expected, actual) in zip(expected_frequencies, frequencies) {
            assert!(evaluate(actual.frequency).approx_eq_with_epsilon(expected.0, 0.0001f64))
        }
        Ok(())
    }

    fn check_frequencies<P: AsRef<Path>>(
        expected_frequencies_path: P,
        scale: &Scale,
        keyboard_mapping: &KeyboardMapping,
    ) -> Result<()> {
        let expected_frequencies = read_expected_frequencies(expected_frequencies_path);
        let frequencies = KeyFrequencyMapping::compute(scale, keyboard_mapping)?;
        assert_eq!(expected_frequencies.len(), frequencies.len());
        for (expected, actual) in zip(expected_frequencies, frequencies) {
            assert!(evaluate(actual.frequency).approx_eq_with_epsilon(expected, 0.000000001f64))
        }
        Ok(())
    }
}
