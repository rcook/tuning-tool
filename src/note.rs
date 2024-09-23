use anyhow::{bail, Error};
use num::{BigRational, One, ToPrimitive};
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Debug)]
pub(crate) enum Note {
    Cents(f64),
    Ratio(BigRational),
}

impl Note {
    pub(crate) fn unison() -> Self {
        Self::Ratio(BigRational::one())
    }

    pub(crate) fn cents(&self) -> f64 {
        match self {
            &Self::Cents(value) => value,
            Self::Ratio(value) => 1200f64 * value.to_f64().expect("TBD").log2(),
        }
    }
}

impl FromStr for Note {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let Some(temp) = s.trim().split_whitespace().next() else {
            bail!("Invalid note specification {s}")
        };

        if temp.contains(".") {
            return Ok(Self::Cents(temp.parse()?));
        }

        Ok(Self::Ratio(temp.parse()?))
    }
}
