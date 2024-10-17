// Copyright (c) 2024 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

use crate::ratio::Ratio;
use anyhow::{bail, Error};
use num::{BigRational, One, ToPrimitive};
use rust_decimal::Decimal;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::str::FromStr;
use tuning_tool_lib::symbolic::Expression;

#[derive(Clone, Debug, PartialEq)]
enum Inner {
    Cents(Decimal),
    Ratio(BigRational),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Interval(Inner);

impl Interval {
    pub(crate) fn unison() -> Self {
        Self(Inner::Ratio(BigRational::one()))
    }

    pub(crate) fn as_ratio_expr(&self) -> Expression {
        match &self.0 {
            Inner::Cents(value) => Expression::new_z(2).pow(
                Expression::try_from(*value).expect("Must be convertible")
                    / Expression::new_z(1200),
            ),
            Inner::Ratio(value) => value.clone().try_into().expect("Must be convertible"),
        }
    }

    pub(crate) fn as_ratio(&self) -> Ratio {
        Ratio(match &self.0 {
            Inner::Cents(value) => 2f64.powf(value.to_f64().expect("Must be f64") / 1200f64),
            Inner::Ratio(value) => value.to_f64().expect("Must be f64"),
        })
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
            bail!("Invalid interval {s}")
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
    use crate::evaluate::Evaluate;
    use crate::interval::Interval;
    use anyhow::Result;

    #[test]
    fn basics() -> Result<()> {
        const EPSILON: f64 = 0.0000001f64;
        assert!(Interval::unison()
            .as_ratio_expr()
            .as_f64()
            .approx_eq_with_epsilon(1f64, EPSILON));
        assert_eq!("1/1", Interval::unison().to_string());

        let interval = "150.5".parse::<Interval>()?;
        assert!(interval
            .as_ratio_expr()
            .as_f64()
            .approx_eq_with_epsilon(1.0908227291337902, EPSILON));
        assert_eq!("150.5", interval.to_string());

        let interval = "19/17".parse::<Interval>()?;
        assert!(interval
            .as_ratio_expr()
            .as_f64()
            .approx_eq_with_epsilon(1.1176470588235294f64, EPSILON));
        assert_eq!("19/17", interval.to_string());

        let interval = "2/1".parse::<Interval>()?;
        assert_eq!(2f64, interval.as_ratio_expr().as_f64());
        assert_eq!("2/1", interval.to_string());

        Ok(())
    }
}
