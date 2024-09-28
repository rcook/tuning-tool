use crate::types::{Cents, Semitones};
use anyhow::{bail, Error};
use num::{BigRational, One, ToPrimitive};
use rust_decimal::Decimal;
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub(crate) enum Interval {
    Cents(Decimal),
    Ratio(BigRational),
}

impl Interval {
    pub(crate) fn unison() -> Self {
        Self::Ratio(BigRational::one())
    }

    pub(crate) fn cents(&self) -> Cents {
        match self {
            &Self::Cents(value) => value.to_f64().expect("Must be f64"),
            Self::Ratio(value) => value.to_f64().expect("Must be f64").log2() * 1200f64,
        }
    }

    #[allow(unused)]
    pub(crate) fn semitones(&self) -> Semitones {
        self.cents() / 100f64
    }
}

impl FromStr for Interval {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let Some(temp) = s.split_whitespace().next() else {
            bail!("Invalid note specification {s}")
        };

        if temp.contains(".") {
            return Ok(Self::Cents(temp.parse()?));
        }

        Ok(Self::Ratio(temp.parse()?))
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
        assert_eq!(Interval::Ratio(BigRational::one()), Interval::unison());

        let note = "150.5".parse::<Interval>()?;
        assert_eq!(Interval::Cents(dec!(150.5)), note);
        assert!(note.cents().approx_eq_with_epsilon(150.50, 0.01));

        let note = "19/17".parse::<Interval>()?;
        assert_eq!(
            Interval::Ratio(BigRational::new(19.into(), 17.into())),
            note
        );
        assert!(note.cents().approx_eq_with_epsilon(192.56, 0.01));

        Ok(())
    }
}
