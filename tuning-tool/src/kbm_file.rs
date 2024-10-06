use crate::frequency::Frequency;
use crate::fs::read_to_string_lossy;
use crate::key_mapping::KeyMapping;
use crate::keyboard_mapping::KeyboardMapping;
use crate::types::KeyNumber;
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
    _middle_key: KeyNumber,
    _equave_degree: usize,
    _key_mappings: Vec<KeyMapping>,
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

        // Start of MIDI key range
        let start_key = read::<KeyNumber, _>(&mut lines)?;
        trace!("Parsed start key {start_key}");

        // End of MIDI key range
        let end_key = read::<KeyNumber, _>(&mut lines)?;
        trace!("Parsed end key {end_key}");

        // Middle key where 1/1 note is mapped
        let middle_key = read::<KeyNumber, _>(&mut lines)?;
        trace!("Parsed middle key {middle_key}");

        // Key where reference frequency goes
        let reference_key = read::<KeyNumber, _>(&mut lines)?;
        trace!("Parsed reference key {reference_key}");

        // Reference frequency (e.g. 440 Hz)
        let reference_frequency = Frequency(read_f64!(lines));
        trace!("Parsed reference frequency {reference_frequency}");

        // Scale interval between adjacent repeating patterns
        let equave_degree = read_usize!(lines);
        trace!("Parsed equave degree {equave_degree}");

        let mut keys = Vec::with_capacity(size);
        for _ in 0..size {
            let s = read_str!(lines);
            let key = if s == "x" {
                KeyMapping::Unmapped
            } else {
                KeyMapping::Degree(s.parse()?)
            };
            trace!("Parsed key mapping {key}");
            keys.push(key);
        }

        if lines.next().is_some() {
            bail!("Invalid .kbm file")
        }

        let keyboard_mapping =
            KeyboardMapping::new(start_key, end_key, reference_key, reference_frequency)?;

        Ok(Self {
            _size: size,
            _middle_key: middle_key,
            _equave_degree: equave_degree,
            _key_mappings: keys,
            keyboard_mapping,
        })
    }
}
