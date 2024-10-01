use crate::cents::Cents;
use crate::semitones::Semitones;
use anyhow::{bail, Error};
use num::{BigRational, One, ToPrimitive};
use rust_decimal::Decimal;
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

    #[allow(unused)]
    pub(crate) fn as_cents(&self) -> Option<&Decimal> {
        match &self.0 {
            Inner::Cents(value) => Some(value),
            _ => None,
        }
    }

    #[allow(unused)]
    pub(crate) fn as_ratio(&self) -> Option<&BigRational> {
        match &self.0 {
            Inner::Ratio(value) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn as_f64(&self) -> f64 {
        match &self.0 {
            Inner::Cents(value) => value.to_f64().expect("Must be f64") / 1200f64,
            Inner::Ratio(value) => value.to_f64().expect("TBD"),
        }
    }

    pub(crate) fn cents(&self) -> Cents {
        Cents(match &self.0 {
            Inner::Cents(value) => value.to_f64().expect("Must be f64"),
            Inner::Ratio(value) => value.to_f64().expect("Must be f64").log2() * 1200f64,
        })
    }

    #[allow(unused)]
    pub(crate) fn semitones(&self) -> Semitones {
        Semitones(self.cents().0 / 100f64)
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
    use num::{BigRational, One};
    use rust_decimal_macros::dec;

    #[test]
    fn basics() -> Result<()> {
        assert_eq!(Some(&BigRational::one()), Interval::unison().as_ratio());

        let interval = "150.5".parse::<Interval>()?;
        assert_eq!(Some(&dec!(150.5)), interval.as_cents());
        assert!(interval.cents().0.approx_eq_with_epsilon(150.50, 0.01));

        let interval = "19/17".parse::<Interval>()?;
        assert_eq!(
            Some(&BigRational::new(19.into(), 17.into())),
            interval.as_ratio()
        );
        assert!(interval.cents().0.approx_eq_with_epsilon(192.56, 0.01));

        Ok(())
    }
}
