use anyhow::{bail, Error};
use num::{BigRational, One, ToPrimitive};
use rust_decimal::Decimal;
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub(crate) enum Note {
    Cents(Decimal),
    Ratio(BigRational),
}

impl Note {
    pub(crate) fn unison() -> Self {
        Self::Ratio(BigRational::one())
    }

    pub(crate) fn cents(&self) -> Option<f64> {
        match self {
            &Self::Cents(value) => value.to_f64(),
            Self::Ratio(value) => value.to_f64().map(|x| 1200f64 * x.log2()),
        }
    }
}

impl FromStr for Note {
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
    use crate::num::ApproxEq;
    use crate::scala::note::Note;
    use anyhow::Result;
    use num::{BigRational, One};
    use rust_decimal_macros::dec;

    #[test]
    fn basics() -> Result<()> {
        assert_eq!(Note::Ratio(BigRational::one()), Note::unison());

        let note = "150.5".parse::<Note>()?;
        assert_eq!(Note::Cents(dec!(150.5)), note);
        assert!(note
            .cents()
            .expect("Must succeed")
            .approx_eq_with_epsilon(150.50, 0.01));

        let note = "19/17".parse::<Note>()?;
        assert_eq!(Note::Ratio(BigRational::new(19.into(), 17.into())), note);
        assert!(note
            .cents()
            .expect("Must succeed")
            .approx_eq_with_epsilon(192.56, 0.01));

        Ok(())
    }
}
