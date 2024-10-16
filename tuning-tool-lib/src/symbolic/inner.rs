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

use crate::maths::IntegerEx;
use crate::symbolic::bracket_style::BracketStyle;
use crate::symbolic::expression::Expression;
use crate::symbolic::value::Value::{self, R, Z};
use std::collections::HashMap;

type E = Box<Expression>;

#[derive(Clone, Debug)]
pub(crate) enum Inner {
    Add(E, E),
    Div(E, E),
    Mul(E, E),
    Pow(E, E),
    Sub(E, E),
    Var(String),
    Val(Value),
    Brackets(E, BracketStyle),
}

impl Inner {
    pub(crate) fn evaluate_with_values(&self, values: &HashMap<&str, Value>) -> Option<Value> {
        match self {
            Self::Add(lhs, rhs) => Some(
                match (
                    lhs.evaluate_with_values(values)?,
                    rhs.evaluate_with_values(values)?,
                ) {
                    (R(lhs), R(rhs)) => R(lhs + rhs),
                    (Z(lhs), Z(rhs)) => Z(lhs.checked_add(rhs).expect("Overflow should not occur")),
                    (R(lhs), Z(rhs)) => R(lhs + rhs as f64),
                    (Z(lhs), R(rhs)) => R(lhs as f64 + rhs),
                },
            ),
            Self::Div(lhs, rhs) => Some(
                match (
                    lhs.evaluate_with_values(values)?,
                    rhs.evaluate_with_values(values)?,
                ) {
                    (R(lhs), R(rhs)) => R(lhs / rhs),
                    (Z(lhs), Z(rhs)) => {
                        match lhs.checked_div_rem(rhs).expect("Overflow should not occur") {
                            (div, 0) => Z(div),
                            _ => R(lhs as f64 / rhs as f64),
                        } // grcov-excl-line
                    } // grcov-excl-line
                    (R(lhs), Z(rhs)) => R(lhs / rhs as f64),
                    (Z(lhs), R(rhs)) => R(lhs as f64 / rhs),
                }, // grcov-excl-line
            ), // grcov-excl-line
            Self::Mul(lhs, rhs) => Some(
                match (
                    lhs.evaluate_with_values(values)?,
                    rhs.evaluate_with_values(values)?,
                ) {
                    (R(lhs), R(rhs)) => R(lhs * rhs),
                    (Z(lhs), Z(rhs)) => Z(lhs.checked_mul(rhs).expect("Overflow should not occur")),
                    (R(lhs), Z(rhs)) => R(lhs * rhs as f64),
                    (Z(lhs), R(rhs)) => R(lhs as f64 * rhs),
                },
            ),
            Self::Pow(lhs, rhs) => Some(
                match (
                    lhs.evaluate_with_values(values)?,
                    rhs.evaluate_with_values(values)?,
                ) {
                    (R(lhs), R(rhs)) => R(lhs.powf(rhs)),
                    (Z(lhs), Z(rhs)) => {
                        if rhs >= 0 {
                            Z(lhs
                                .checked_pow(rhs as u32)
                                .expect("Overflow should not occur"))
                        } else {
                            R((lhs as f64).powi(rhs))
                        }
                    }
                    (R(lhs), Z(rhs)) => R(lhs.powi(rhs)),
                    (Z(lhs), R(rhs)) => R((lhs as f64).powf(rhs)),
                },
            ),
            Self::Sub(lhs, rhs) => Some(
                match (
                    lhs.evaluate_with_values(values)?,
                    rhs.evaluate_with_values(values)?,
                ) {
                    (R(lhs), R(rhs)) => R(lhs - rhs),
                    (Z(lhs), Z(rhs)) => Z(lhs.checked_sub(rhs).expect("Overflow should not occur")),
                    (R(lhs), Z(rhs)) => R(lhs - rhs as f64),
                    (Z(lhs), R(rhs)) => R(lhs as f64 - rhs),
                },
            ),
            Self::Var(name) => values.get(name.as_str()).cloned(),
            Self::Val(value) => Some(value.clone()),
            Self::Brackets(e, _) => e.evaluate_with_values(values),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::symbolic::expression::Expression;
    use crate::symbolic::inner::Inner::{self, *};
    use crate::symbolic::value::Value::{self, *};
    use rstest::rstest;
    use std::collections::HashMap;

    macro_rules! val {
        ($value: expr) => {
            Box::new(Expression::new_val($value))
        };
    }

    #[rstest]
    #[case(R(3f64), Add(val!(R(1f64)), val!(R(2f64))))]
    #[case(Z(3), Add(val!(Z(1)), val!(Z(2))))]
    #[case(R(3f64), Add(val!(R(1f64)), val!(Z(2))))]
    #[case(R(3f64), Add(val!(Z(1)), val!(R(2f64))))]
    #[case(R(0.5f64), Div(val!(R(1f64)), val!(R(2f64))))]
    #[case(R(0.5f64), Div(val!(Z(1)), val!(Z(2))))]
    #[case(Z(2), Div(val!(Z(2)), val!(Z(1))))]
    #[case(R(0.5f64), Div(val!(R(1f64)), val!(Z(2))))]
    #[case(R(0.5f64), Div(val!(Z(1)), val!(R(2f64))))]
    #[case(R(2f64), Mul(val!(R(1f64)), val!(R(2f64))))]
    #[case(Z(2), Mul(val!(Z(1)), val!(Z(2))))]
    #[case(R(2f64), Mul(val!(R(1f64)), val!(Z(2))))]
    #[case(R(2f64), Mul(val!(Z(1)), val!(R(2f64))))]
    #[case(R(1f64), Pow(val!(R(1f64)), val!(R(2f64))))]
    #[case(Z(1), Pow(val!(Z(1)), val!(Z(2))))]
    #[case(R(0.25f64), Pow(val!(Z(2)), val!(Z(-2))))]
    #[case(R(1f64), Pow(val!(R(1f64)), val!(Z(2))))]
    #[case(R(1f64), Pow(val!(Z(1)), val!(R(2f64))))]
    #[case(R(-1f64), Sub(val!(R(1f64)), val!(R(2f64))))]
    #[case(Z(-1), Sub(val!(Z(1)), val!(Z(2))))]
    #[case(R(-1f64), Sub(val!(R(1f64)), val!(Z(2))))]
    #[case(R(-1f64), Sub(val!(Z(1)), val!(R(2f64))))]
    fn basics(#[case] expected: Value, #[case] expr: Inner) {
        assert_eq!(
            Some(expected),
            expr.evaluate_with_values(&HashMap::from([]))
        );
    }
}
