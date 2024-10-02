use crate::cents::Cents;
use crate::semitones::Semitones;
use anyhow::{bail, Error};
use num::{BigRational, One, ToPrimitive};
use rust_decimal::Decimal;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Inner {
    Cents(Decimal),
    Ratio(BigRational),
}

#[derive(Debug, PartialEq)]
pub(crate) struct Interval(Inner);

impl Interval {
    pub(crate) fn unison() -> Self {
        Self(Inner::Ratio(BigRational::one()))
    }

    pub(crate) fn as_ratio(&self) -> f64 {
        match &self.0 {
            Inner::Cents(value) => 2f64.powf(value.to_f64().expect("Must be f64") / 1200f64),
            Inner::Ratio(value) => value.to_f64().expect("Must be f64"),
        }
    }

    pub(crate) fn as_cents(&self) -> Cents {
        Cents(match &self.0 {
            Inner::Cents(value) => value.to_f64().expect("Must be f64"),
            Inner::Ratio(value) => value.to_f64().expect("Must be f64").log2() * 1200f64,
        })
    }

    #[allow(unused)]
    pub(crate) fn semitones(&self) -> Semitones {
        Semitones(self.as_cents().0 / 100f64)
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.0 {
            Inner::Cents(value) => write!(f, "{}", value),
            Inner::Ratio(value) => write!(f, "{}/{}", value.numer(), value.denom()),
        }
    }
}

impl FromStr for Interval {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let Some(temp) = s.split_whitespace().next() else {
            bail!("Invalid note specification {s}")
        };

        if temp.contains(".") {
            return Ok(Self(Inner::Cents(temp.parse()?)));
        }

        Ok(Self(Inner::Ratio(temp.parse()?)))
    }
}

#[cfg(test)]
mod tests {
    use crate::approx_eq::ApproxEq;
    use crate::interval::Interval;
    use anyhow::Result;

    #[test]
    fn basics() -> Result<()> {
        const EPSILON: f64 = 0.0000001f64;
        assert!(Interval::unison()
            .as_ratio()
            .approx_eq_with_epsilon(1f64, EPSILON));
        assert!(Interval::unison()
            .as_cents()
            .0
            .approx_eq_with_epsilon(0f64, EPSILON));
        assert_eq!("1/1", Interval::unison().to_string());

        let interval = "150.5".parse::<Interval>()?;
        assert!(interval
            .as_ratio()
            .approx_eq_with_epsilon(1.0908227291337902, EPSILON));
        assert_eq!(150.5f64, interval.as_cents().0);
        assert!(interval
            .as_cents()
            .0
            .approx_eq_with_epsilon(150.50f64, 0.01f64));
        assert_eq!("150.5", interval.to_string());

        let interval = "19/17".parse::<Interval>()?;
        assert!(interval
            .as_ratio()
            .approx_eq_with_epsilon(1.1176470588235294f64, EPSILON));
        assert!(interval
            .as_cents()
            .0
            .approx_eq_with_epsilon(192.55760663189534, EPSILON));
        assert_eq!("19/17", interval.to_string());

        Ok(())
    }
}
