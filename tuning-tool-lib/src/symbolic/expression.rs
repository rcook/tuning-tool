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

use crate::symbolic::brackets::Brackets;
use crate::symbolic::op::Op;
use crate::symbolic::value::Value::{self, R, Z};
use num::Integer;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Debug)]
pub struct Expression {
    operands: Vec<Self>,
    op: Op,
}

impl Expression {
    #[allow(unused)]
    pub fn new(lhs: Self, rhs: Self, op: Op) -> Self {
        Self {
            operands: vec![lhs, rhs],
            op,
        }
    }

    #[allow(unused)]
    pub fn new_var(name: &str) -> Self {
        Self {
            operands: vec![],
            op: Op::Var(String::from(name)),
        }
    }

    #[allow(unused)]
    pub fn new_val(value: Value) -> Self {
        Self {
            operands: vec![],
            op: Op::Val(value),
        }
    }

    #[allow(unused)]
    pub fn new_r(value: f64) -> Self {
        Self {
            operands: vec![],
            op: Op::Val(R(value)),
        }
    }

    #[allow(unused)]
    pub fn new_z(value: i32) -> Self {
        Self {
            operands: vec![],
            op: Op::Val(Z(value)),
        }
    }

    #[allow(unused)]
    pub fn new_brackets(expression: Self, brackets: Brackets) -> Self {
        Self {
            operands: vec![],
            op: Op::Brackets(Box::new(expression), brackets),
        }
    }

    #[allow(unused)]
    pub fn brackets(self, brackets: Brackets) -> Self {
        Self::new_brackets(self, brackets)
    }

    #[allow(unused)]
    pub fn brackets_round(self) -> Self {
        self.brackets(Brackets::Round)
    }

    #[allow(unused)]
    pub fn pow(mut self, mut rhs: Self) -> Self {
        if self.op < Op::Pow {
            self = self.brackets_round()
        };
        if rhs.op < Op::Pow {
            rhs = rhs.brackets_round()
        };

        Self::new(self, rhs, Op::Pow)
    }

    #[allow(unused)]
    pub fn evaluate(&self) -> Option<Value> {
        self.evaluate_with_values(&HashMap::from([]))
    }

    #[allow(unused)]
    pub fn evaluate_with_values(&self, values: &HashMap<&str, Value>) -> Option<Value> {
        match &self.op {
            Op::Add => Some(
                match (
                    self.operands[0].evaluate_with_values(values)?,
                    self.operands[1].evaluate_with_values(values)?,
                ) {
                    (R(lhs), R(rhs)) => R(lhs + rhs),
                    (Z(lhs), Z(rhs)) => Z(lhs + rhs),
                    (R(lhs), Z(rhs)) => R(lhs + rhs as f64),
                    (Z(lhs), R(rhs)) => R(lhs as f64 + rhs),
                },
            ),
            Op::Div => Some(
                match (
                    self.operands[0].evaluate_with_values(values)?,
                    self.operands[1].evaluate_with_values(values)?,
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
            Op::Mul => Some(
                match (
                    self.operands[0].evaluate_with_values(values)?,
                    self.operands[1].evaluate_with_values(values)?,
                ) {
                    (R(lhs), R(rhs)) => R(lhs * rhs),
                    (Z(lhs), Z(rhs)) => Z(lhs * rhs),
                    (R(lhs), Z(rhs)) => R(lhs * rhs as f64),
                    (Z(lhs), R(rhs)) => R(lhs as f64 * rhs),
                },
            ),
            Op::Pow => Some(
                match (
                    self.operands[0].evaluate_with_values(values)?,
                    self.operands[1].evaluate_with_values(values)?,
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
            Op::Sub => Self::new(self.operands[0].clone(), -self.operands[1].clone(), Op::Add)
                .evaluate_with_values(values),
            Op::Var(var) => values.get(var.as_str()).cloned(),
            Op::Val(value) => Some(value.clone()),
            Op::Brackets(e, _) => e.evaluate_with_values(values),
        }
    }

    pub(crate) fn empty() -> Self {
        Self {
            operands: vec![],
            op: Op::Add,
        }
    }

    fn brace_if(self, target: Op) -> Self {
        if self.op < target {
            self.brackets_round()
        } else {
            self
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.op.clone() {
            Op::Val(value) => f.write_str(&value.to_string()),
            Op::Var(literal) => f.write_str(&literal),
            Op::Brackets(expr, brackets) => {
                let (left, right) = brackets.get_symbols();
                f.write_str(left)?;
                f.write_str(&expr.to_string())?;
                f.write_str(right)
            }
            _ => {
                f.write_str(&self.operands[0].to_string())?;
                f.write_str(&self.op.to_string())?;
                f.write_str(&self.operands[1].to_string())
            }
        }
    }
}

impl Add for Expression {
    type Output = Expression;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        self = self.brace_if(Op::Add);
        rhs = rhs.brace_if(Op::Add);
        Expression::new(self, rhs, Op::Add)
    }
}

impl Div for Expression {
    type Output = Self;

    fn div(mut self, mut rhs: Self) -> Self::Output {
        self = self.brace_if(Op::Div);
        rhs = rhs.brace_if(Op::Div);
        Expression::new(self, rhs, Op::Div)
    }
}

impl Mul for Expression {
    type Output = Expression;

    fn mul(mut self, mut rhs: Self) -> Self::Output {
        self = self.brace_if(Op::Mul);
        rhs = rhs.brace_if(Op::Mul);
        Expression::new(self, rhs, Op::Mul)
    }
}

impl Neg for Expression {
    type Output = Expression;

    fn neg(self) -> Self::Output {
        Expression::new(self, Expression::new_z(-1), Op::Mul)
    }
}

impl Sub for Expression {
    type Output = Expression;

    fn sub(mut self, mut rhs: Self) -> Self::Output {
        self = self.brace_if(Op::Add);
        rhs = rhs.brace_if(Op::Add);
        Expression::new(self, rhs, Op::Sub)
    }
}

#[cfg(test)]
mod tests {
    use crate::symbolic::expression::Expression;
    use crate::symbolic::value::Value::{self, R, Z};
    use rstest::rstest;
    use std::collections::HashMap;

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
}
