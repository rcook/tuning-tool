use anyhow::{bail, Error};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;

pub(crate) struct Frequency(f64);

impl Frequency {
    pub(crate) const fn concert_a() -> Self {
        Self(440f64)
    }

    #[must_use]
    pub(crate) const fn to_f64(&self) -> f64 {
        self.0
    }
}

impl Display for Frequency {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{value} Hz", value = self.0)
    }
}

impl From<f64> for Frequency {
    fn from(value: f64) -> Self {
        Self(value)
    }
}
