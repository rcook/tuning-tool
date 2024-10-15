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

use crate::symbolic::bracket_style::BracketStyle;
use crate::symbolic::expression::Expression;
use crate::symbolic::value::Value::{self, R, Z};
use num::Integer;
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
                    (Z(lhs), Z(rhs)) => Z(lhs + rhs),
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
                    (Z(lhs), Z(rhs)) => match lhs.div_rem(&rhs) {
                        (div, 0) => Z(div),
                        _ => R(lhs as f64 / rhs as f64),
                    },
                    (R(lhs), Z(rhs)) => R(lhs / rhs as f64),
                    (Z(lhs), R(rhs)) => R(lhs as f64 / rhs),
                },
            ),
            Self::Mul(lhs, rhs) => Some(
                match (
                    lhs.evaluate_with_values(values)?,
                    rhs.evaluate_with_values(values)?,
                ) {
                    (R(lhs), R(rhs)) => R(lhs * rhs),
                    (Z(lhs), Z(rhs)) => Z(lhs * rhs),
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
                            Z(lhs.pow(rhs as u32))
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
                    (Z(lhs), Z(rhs)) => Z(lhs - rhs),
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
