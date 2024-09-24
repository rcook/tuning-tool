use anyhow::{bail, Error};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;

pub(crate) struct Hertz(f64);

impl Hertz {
    #[must_use]
    pub(crate) const fn new(value: f64) -> Self {
        Self(value)
    }

    #[must_use]
    pub(crate) const fn to_f64(&self) -> f64 {
        self.0
    }
}

impl Display for Hertz {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{value} Hz", value = self.0)
    }
}

impl TryFrom<f64> for Hertz {
    type Error = Error;

    fn try_from(value: f64) -> StdResult<Self, Self::Error> {
        if value < 0f64 {
            bail!("Invalid frequency {value}")
        }

        Ok(Self(value))
    }
}
