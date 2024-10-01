use crate::ascii_char::AsciiChar;
use anyhow::{bail, Error};
use std::result::Result as StdResult;
use std::str::FromStr;

const PRESET_NAME_LEN: usize = 16;

type PresetNameArray = [AsciiChar; PRESET_NAME_LEN];

#[derive(Debug)]
pub(crate) struct PresetName(PresetNameArray);

impl PresetName {
    pub(crate) const LEN: usize = PRESET_NAME_LEN;

    pub(crate) const fn new(slice: PresetNameArray) -> Self {
        Self(slice)
    }

    pub(crate) const fn as_array(&self) -> &PresetNameArray {
        &self.0
    }
}

impl FromStr for PresetName {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        if !s.is_ascii() {
            bail!("Preset name is not ASCII")
        }
        if s.len() > Self::LEN {
            bail!("Invalid preset name")
        }

        // Terribly inefficient, but at least it's safe!
        let mut array = [AsciiChar::ZERO; Self::LEN];
        for (i, c) in s.bytes().enumerate() {
            array[i] = c.try_into()?;
        }

        Ok(Self(array))
    }
}
