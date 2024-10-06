use crate::frequency::Frequency;
use crate::fs::read_to_string_lossy;
use crate::key::Key;
use crate::keyboard_mapping::KeyboardMapping;
use crate::note_number::NoteNumber;
use anyhow::bail;
use anyhow::{Error, Result};
use log::trace;
use std::path::Path;
use std::result::Result as StdResult;
use std::str::FromStr;
use tuning_tool_lib::U7;

macro_rules! read_str {
    ($iter: expr) => {
        $iter
            .next()
            .ok_or_else(|| ::anyhow::anyhow!("Stream is exhausted"))?
    };
}

fn read<'a, U, I>(iter: &mut I) -> Result<U>
where
    U: U7,
    I: Iterator<Item = &'a str>,
{
    let s = read_str!(iter);
    let value = s.parse::<u8>()?;
    Ok(value.try_into()?)
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
    _size: usize,
    _middle_note_number: NoteNumber,
    _octave_degree: usize,
    _keys: Vec<Key>,
    keyboard_mapping: KeyboardMapping,
}

impl KbmFile {
    pub(crate) fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        trace!("Reading .kbm file {path}", path = path.as_ref().display());
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

        let size = read_str!(lines).parse::<usize>()?;
        if size > 127 {
            bail!("Invalid size")
        }

        let start_note_number = read::<NoteNumber, _>(&mut lines)?;
        let end_note_number = read::<NoteNumber, _>(&mut lines)?;
        let middle_note_number = read::<NoteNumber, _>(&mut lines)?;
        let base_note_number = read::<NoteNumber, _>(&mut lines)?;
        let base_frequency = Frequency(read_f64!(lines));
        let octave_degree = read_usize!(lines);

        let mut keys = Vec::with_capacity(size);
        for _ in 0..size {
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

        if octave_degree != size {
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
