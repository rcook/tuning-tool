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
use crate::kbm_file::KbmFile;
use crate::key_mapping::KeyMapping;
use crate::key_mappings::KeyMappings;
use crate::keyboard_mapping::KeyboardMapping;
use crate::midi_note::MidiNote;
use crate::scale::Scale;
use crate::tuning_tool_args::KeyboardMappingSourceGroup;
use crate::types::KeyNumber;
use anyhow::Result;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::PathBuf;

pub(crate) enum KeyboardMappingSource {
    KbmFile(PathBuf),
    Linear,
    WhiteKeys,
}

impl KeyboardMappingSource {
    pub(crate) fn make_keyboard_mapping(&self, scale: &Scale) -> Result<KeyboardMapping> {
        match self {
            Self::KbmFile(kbm_path) => {
                let kbm_file = KbmFile::read(kbm_path)?;
                Ok(kbm_file.keyboard_mapping().clone())
            }
            Self::Linear => {
                let start_key = KeyNumber::MIN;
                let end_key = KeyNumber::MAX;
                let zero_key = KeyNumber::constant::<69>();
                let reference_key = zero_key;
                let reference_frequency = Frequency::CONCERT_A4;
                let key_mappings = KeyMappings::Linear;
                KeyboardMapping::new(
                    start_key,
                    end_key,
                    zero_key,
                    reference_key,
                    reference_frequency,
                    key_mappings,
                )
            }
            Self::WhiteKeys => {
                let interval_count = scale.intervals().len();
                if interval_count != 7 {
                    todo!("--white not implemented for interval count {interval_count}");
                }

                let start_key = KeyNumber::MIN;
                let end_key = KeyNumber::MAX;
                let zero_key = KeyNumber::constant::<69>();
                let reference_key = zero_key;
                let reference_frequency = Frequency::CONCERT_A4;

                let mut degree = 0;
                let key_mappings = KeyMappings::Custom(
                    MidiNote::ALL
                        .iter()
                        .skip(zero_key.to_u8() as usize)
                        .take(12)
                        .map(|n| {
                            if n.is_natural() {
                                let key_mapping = KeyMapping::Degree(degree);
                                degree += 1;
                                key_mapping
                            } else {
                                KeyMapping::Unmapped
                            }
                        })
                        .collect::<Vec<_>>(),
                );

                KeyboardMapping::new(
                    start_key,
                    end_key,
                    zero_key,
                    reference_key,
                    reference_frequency,
                    key_mappings,
                )
            }
        }
    }
}

impl Display for KeyboardMappingSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::KbmFile(kbm_path) => write!(
                f,
                "Keyboard mapping file: {path}",
                path = kbm_path.display()
            ),
            Self::Linear => write!(f, "Linear"),
            Self::WhiteKeys => write!(f, "White notes"),
        }
    }
}

impl From<KeyboardMappingSourceGroup> for KeyboardMappingSource {
    fn from(value: KeyboardMappingSourceGroup) -> Self {
        match (value.kbm_path, value.linear, value.white_keys) {
            (Some(kbm_path), false, false) => Self::KbmFile(kbm_path),
            (None, true, false) => Self::Linear,
            (None, false, true) => Self::WhiteKeys,
            _ => unreachable!(),
        }
    }
}
