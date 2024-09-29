use crate::consts::U7_ZERO;
use anyhow::{anyhow, bail, Error};
use midly::num::u7;
use std::result::Result as StdResult;
use std::str::FromStr;

const PRESET_NAME_LEN: usize = 16;

type PresetNameArray = [u7; PRESET_NAME_LEN];

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
        let slice = u7::slice_try_from_int(s.as_bytes())
            .ok_or_else(|| anyhow!("Preset name is not ASCII"))?;
        if slice.len() > Self::LEN {
            bail!("Invalid preset name")
        }

        let mut array = [U7_ZERO; Self::LEN];
        array[0..slice.len()].copy_from_slice(slice);
        Ok(Self(array))
    }
}
