use crate::frequency::Frequency;
use crate::fs::read_to_string_lossy;
use crate::key::Key;
use crate::keyboard_mapping::KeyboardMapping;
use crate::note_number::NoteNumber;
use anyhow::bail;
use anyhow::{Error, Result};
use midly::num::u7;
use std::path::Path;
use std::result::Result as StdResult;
use std::str::FromStr;

macro_rules! read_str {
    ($iter: expr) => {
        $iter
            .next()
            .ok_or_else(|| ::anyhow::anyhow!("Stream is exhausted"))?
    };
}

macro_rules! read_u7 {
    ($iter: expr) => {{
        let s = read_str!($iter);
        let value = s.parse::<u8>()?;
        let value: ::midly::num::u7 = value.try_into()?;
        value
    }};
}

macro_rules! read_f64 {
    ($iter: expr) => {{
        let s = read_str!($iter);
        let value = s.parse::<f64>()?;
        value
    }};
}

macro_rules! read_usize {
    ($iter: expr) => {{
        let s = read_str!($iter);
        let value = s.parse::<usize>()?;
        value
    }};
}

#[derive(Debug)]
pub(crate) struct KbmFile {
    _size: u7,
    _middle_note_number: NoteNumber,
    _octave_degree: usize,
    _keys: Vec<Key>,
    keyboard_mapping: KeyboardMapping,
}

impl KbmFile {
    pub(crate) fn read(path: &Path) -> Result<Self> {
        read_to_string_lossy(path)?.parse()
    }

    pub(crate) const fn keyboard_mapping(&self) -> &KeyboardMapping {
        &self.keyboard_mapping
    }
}

impl FromStr for KbmFile {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let mut lines = s.lines().filter_map(|line| {
            let s = line.trim();
            if s.is_empty() || s.starts_with("!") {
                None
            } else {
                Some(s)
            }
        });

        let size = read_u7!(lines);
        let start_note_number = NoteNumber(read_u7!(lines));
        let end_note_number = NoteNumber(read_u7!(lines));
        let middle_note_number = NoteNumber(read_u7!(lines));
        let base_note_number = NoteNumber(read_u7!(lines));
        let base_frequency = Frequency(read_f64!(lines));
        let octave_degree = read_usize!(lines);

        let count = size.as_int() as usize;
        let mut keys = Vec::with_capacity(count);
        for _ in 0..count {
            let s = read_str!(lines);
            keys.push(if s == "x" {
                Key::Unmapped
            } else {
                Key::Degree(s.parse()?)
            })
        }

        if lines.next().is_some() {
            bail!("Invalid .kbm file")
        }

        let keyboard_mapping = KeyboardMapping::new(
            start_note_number,
            end_note_number,
            base_note_number,
            base_frequency,
        )?;

        // <LIMITATIONS>

        if octave_degree != 12 {
            todo!("Non-octave scales not currently supported");
        }

        for (i, key) in keys.iter().enumerate() {
            let Key::Degree(degree) = key else {
                todo!("Nontrivial keyboard mappings not yet supported");
            };
            if *degree != i {
                todo!("Nontrivial keyboard mappings not yet supported");
            }
        }

        // </LIMITATIONS>

        Ok(Self {
            _size: size,
            _middle_note_number: middle_note_number,
            _octave_degree: octave_degree,
            _keys: keys,
            keyboard_mapping,
        })
    }
}
