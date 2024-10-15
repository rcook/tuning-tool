// Copyright (c) 2024 Richard Cook and others
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

// Inspired by https://github.com/simensgreen/rusymbols

use crate::error::{TryFromDecimalError, TryFromRatioError};
use crate::symbolic::bracket_style::BracketStyle;
use crate::symbolic::consts::{SYMBOL_ADD, SYMBOL_DIV, SYMBOL_MUL, SYMBOL_POW, SYMBOL_SUB};
use crate::symbolic::inner::Inner;
use crate::symbolic::op::Op;
use crate::symbolic::value::Value::{self, R, Z};
use num::rational::Ratio;
use num::{BigInt, ToPrimitive};
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::result::Result as StdResult;

#[derive(Clone, Debug)]
pub struct Expression(Inner, Op);

impl Expression {
    #[allow(unused)]
    pub fn new_var(name: &str) -> Self {
        Self(Inner::Var(String::from(name)), Op::Var)
    }

    #[allow(unused)]
    pub fn new_val(value: Value) -> Self {
        Self(Inner::Val(value), Op::Val)
    }

    #[allow(unused)]
    pub fn new_r(value: f64) -> Self {
        Self(Inner::Val(R(value)), Op::Val)
    }

    #[allow(unused)]
    pub fn new_z(value: i32) -> Self {
        Self(Inner::Val(Z(value)), Op::Val)
    }

    #[allow(unused)]
    pub fn new_brackets(expression: Self, brackets: BracketStyle) -> Self {
        Self(
            Inner::Brackets(Box::new(expression), brackets),
            Op::Brackets,
        )
    }

    #[allow(unused)]
    pub fn brackets(self, brackets: BracketStyle) -> Self {
        Self::new_brackets(self, brackets)
    }

    #[allow(unused)]
    pub fn brackets_round(self) -> Self {
        self.brackets(BracketStyle::Round)
    }

    #[allow(unused)]
    pub fn pow(mut self, mut rhs: Self) -> Self {
        if self.1.is_lower_precedence_than(&Op::Pow) {
            self = self.brackets_round()
        }
        if rhs.1.is_lower_precedence_than(&Op::Pow) {
            rhs = rhs.brackets_round()
        }
        Self(Inner::Pow(Box::new(self), Box::new(rhs)), Op::Pow)
    }

    #[allow(unused)]
    pub fn evaluate(&self) -> Option<Value> {
        self.evaluate_with_values(&HashMap::from([]))
    }

    #[allow(unused)]
    pub fn evaluate_with_values(&self, values: &HashMap<&str, Value>) -> Option<Value> {
        self.0.evaluate_with_values(values)
    }

    fn brace_if(self, target: Op) -> Self {
        if self.1.is_lower_precedence_than(&target) {
            self.brackets_round()
        } else {
            self
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.0 {
            Inner::Add(lhs, rhs) => write!(f, "{lhs} {SYMBOL_ADD} {rhs}"),
            Inner::Div(lhs, rhs) => write!(f, "{lhs} {SYMBOL_DIV} {rhs}"),
            Inner::Mul(lhs, rhs) => write!(f, "{lhs} {SYMBOL_MUL} {rhs}"),
            Inner::Pow(lhs, rhs) => write!(f, "{lhs} {SYMBOL_POW} {rhs}"),
            Inner::Sub(lhs, rhs) => write!(f, "{lhs} {SYMBOL_SUB} {rhs}"),
            Inner::Var(name) => write!(f, "{name}"),
            Inner::Val(value) => write!(f, "{value}"),
            Inner::Brackets(e, brackets) => {
                let (open, close) = brackets.get_delimiters();
                write!(f, "{open}{e}{close}")
            }
        }
    }
}

impl Add for Expression {
    type Output = Self;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        self = self.brace_if(Op::Add);
        rhs = rhs.brace_if(Op::Add);
        Self(Inner::Add(Box::new(self), Box::new(rhs)), Op::Add)
    }
}

impl Div for Expression {
    type Output = Self;

    fn div(mut self, mut rhs: Self) -> Self::Output {
        self = self.brace_if(Op::Div);
        rhs = rhs.brace_if(Op::Div);
        Self(Inner::Div(Box::new(self), Box::new(rhs)), Op::Div)
    }
}

impl Mul for Expression {
    type Output = Self;

    fn mul(mut self, mut rhs: Self) -> Self::Output {
        self = self.brace_if(Op::Mul);
        rhs = rhs.brace_if(Op::Mul);
        Self(Inner::Mul(Box::new(self), Box::new(rhs)), Op::Mul)
    }
}

impl Neg for Expression {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(
            Inner::Mul(Box::new(self), Box::new(Expression::new_z(-1))),
            Op::Mul,
        )
    }
}

impl Sub for Expression {
    type Output = Self;

    fn sub(mut self, mut rhs: Self) -> Self::Output {
        self = self.brace_if(Op::Add);
        rhs = rhs.brace_if(Op::Add);
        Self(Inner::Sub(Box::new(self), Box::new(rhs)), Op::Sub)
    }
}

impl TryFrom<Decimal> for Expression {
    type Error = TryFromDecimalError;

    fn try_from(value: Decimal) -> StdResult<Self, Self::Error> {
        Ok(if value.is_integer() {
            Expression::new_z(
                value
                    .to_i32()
                    .ok_or(TryFromDecimalError::CouldNotConvert(value))?,
            )
        } else {
            Expression::new_r(
                value
                    .to_f64()
                    .ok_or(TryFromDecimalError::CouldNotConvert(value))?,
            )
        })
    }
}

impl TryFrom<Ratio<BigInt>> for Expression {
    type Error = TryFromRatioError;

    fn try_from(value: Ratio<BigInt>) -> StdResult<Self, Self::Error> {
        let numer = value
            .numer()
            .to_i32()
            .ok_or_else(|| TryFromRatioError::CouldNotConvert(value.clone()))?;
        let denom = value
            .denom()
            .to_i32()
            .ok_or_else(|| TryFromRatioError::CouldNotConvert(value.clone()))?;
        Ok(Expression::new_z(numer) / Expression::new_z(denom))
    }
}

#[cfg(test)]
mod tests {
    use crate::symbolic::bracket_style::BracketStyle;
    use crate::symbolic::expression::Expression;
    use crate::symbolic::value::Value::{self, R, Z};
    use rstest::rstest;
    use std::collections::HashMap;
    use std::ops::{Add, Div};

    #[rstest]
    #[case(3f64, 1f64, 2f64)]
    fn add_r(#[case] expected: f64, #[case] lhs: f64, #[case] rhs: f64) {
        let e0 = Expression::new_r(lhs);
        let e1 = Expression::new_r(rhs);
        let e2 = e0 + e1;
        assert_eq!(Some(R(expected)), e2.evaluate());
    }

    #[rstest]
    #[case(4f64, 100f64, 25f64)]
    fn div_r(#[case] expected: f64, #[case] lhs: f64, #[case] rhs: f64) {
        let e0 = Expression::new_r(lhs);
        let e1 = Expression::new_r(rhs);
        let e2 = e0 / e1;
        assert_eq!(Some(R(expected)), e2.evaluate());
    }

    #[rstest]
    #[case(2500f64, 100f64, 25f64)]
    fn mul_r(#[case] expected: f64, #[case] lhs: f64, #[case] rhs: f64) {
        let e0 = Expression::new_r(lhs);
        let e1 = Expression::new_r(rhs);
        let e2 = e0 * e1;
        assert_eq!(Some(R(expected)), e2.evaluate());
    }

    #[rstest]
    #[case(-100f64, 100f64)]
    #[case(100f64, -100f64)]
    fn neg_r(#[case] expected: f64, #[case] operand: f64) {
        let e0 = Expression::new_r(operand);
        let e1 = -e0;
        assert_eq!(Some(R(expected)), e1.evaluate());
    }

    #[rstest]
    #[case(100f64, 10f64, 2f64)]
    fn pow_r(#[case] expected: f64, #[case] lhs: f64, #[case] rhs: f64) {
        let e0 = Expression::new_r(lhs);
        let e1 = Expression::new_r(rhs);
        let e2 = e0.pow(e1);
        assert_eq!(Some(R(expected)), e2.evaluate());
    }

    #[rstest]
    #[case(-1f64, 1f64, 2f64)]
    fn sub_r(#[case] expected: f64, #[case] lhs: f64, #[case] rhs: f64) {
        let e0 = Expression::new_r(lhs);
        let e1 = Expression::new_r(rhs);
        let e2 = e0 - e1;
        assert_eq!(Some(R(expected)), e2.evaluate());
    }

    #[rstest]
    #[case(3, 1, 2)]
    fn add_z(#[case] expected: i32, #[case] lhs: i32, #[case] rhs: i32) {
        let e0 = Expression::new_z(lhs);
        let e1 = Expression::new_z(rhs);
        let e2 = e0 + e1;
        assert_eq!(Some(Z(expected)), e2.evaluate());
    }

    #[rstest]
    #[case(R(0.5f64), 1, 2)]
    #[case(Z(2), 2, 1)]
    #[case(Z(-2), -2, 1)]
    #[case(Z(-2), 2, -1)]
    #[case(Z(2), -2, -1)]
    #[test]
    fn div_z(#[case] expected: Value, #[case] lhs: i32, #[case] rhs: i32) {
        let e0 = Expression::new_z(lhs);
        let e1 = Expression::new_z(rhs);
        let e2 = e0 / e1;
        assert_eq!(Some(expected), e2.evaluate());
    }

    #[rstest]
    #[case(2, 1, 2)]
    fn mul_z(#[case] expected: i32, #[case] lhs: i32, #[case] rhs: i32) {
        let e0 = Expression::new_z(lhs);
        let e1 = Expression::new_z(rhs);
        let e2 = e0 * e1;
        assert_eq!(Some(Z(expected)), e2.evaluate());
    }

    #[rstest]
    #[case(-100, 100)]
    #[case(100, -100)]
    fn neg_z(#[case] expected: i32, #[case] operand: i32) {
        let e0 = Expression::new_z(operand);
        let e1 = -e0;
        assert_eq!(Some(Z(expected)), e1.evaluate());
    }

    #[rstest]
    #[case(100, 10, 2)]
    fn pow_z(#[case] expected: i32, #[case] lhs: i32, #[case] rhs: i32) {
        let e0 = Expression::new_z(lhs);
        let e1 = Expression::new_z(rhs);
        let e2 = e0.pow(e1);
        assert_eq!(Some(Z(expected)), e2.evaluate());
    }

    #[rstest]
    #[case(-1, 1, 2)]
    fn sub_z(#[case] expected: i32, #[case] lhs: i32, #[case] rhs: i32) {
        let e0 = Expression::new_z(lhs);
        let e1 = Expression::new_z(rhs);
        let e2 = e0 - e1;
        assert_eq!(Some(Z(expected)), e2.evaluate());
    }

    #[rstest]
    #[case(Z(2), Z(10), Z(5))]
    #[case(R(0.5f64), Z(5), Z(10))]
    fn vars(#[case] expected: Value, #[case] x: Value, #[case] rhs: Value) {
        let e0 = Expression::new_var("x");
        let e1 = Expression::new_val(rhs);
        let e2 = e0 / e1;
        assert_eq!(
            Some(expected),
            e2.evaluate_with_values(&HashMap::from([("x", x)]))
        );
    }

    #[test]
    fn precedence() {
        let e0 = Expression::new_z(10);
        let e1 = Expression::new_z(20);
        let e2 = e0.add(e1);
        let e3 = Expression::new_z(2);
        let e4 = e2.div(e3);
        assert_eq!(Some(Z(15)), e4.evaluate());
    }

    #[test]
    fn display() {
        assert_eq!(
            "2 + 3",
            (Expression::new_z(2) + Expression::new_z(3)).to_string()
        );
        assert_eq!(
            "2 / 3",
            (Expression::new_z(2) / Expression::new_z(3)).to_string()
        );
        assert_eq!(
            "2 * 3",
            (Expression::new_z(2) * Expression::new_z(3)).to_string()
        );
        assert_eq!(
            "2 ** 3",
            Expression::new_z(2).pow(Expression::new_z(3)).to_string()
        );
        assert_eq!(
            "2 - 3",
            (Expression::new_z(2) - Expression::new_z(3)).to_string()
        );
        assert_eq!("x", Expression::new_var("x").to_string());
        assert_eq!(
            "{2 + 3}",
            Expression::brackets(
                Expression::new_z(2) + Expression::new_z(3),
                BracketStyle::Curly
            )
            .to_string()
        );
    }
}
