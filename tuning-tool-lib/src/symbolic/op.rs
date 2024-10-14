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
use crate::symbolic::expression::Expression;
use crate::symbolic::literals::{SYMBOL_ADD, SYMBOL_DIV, SYMBOL_MUL, SYMBOL_POW, SYMBOL_SUB};
use crate::symbolic::value::Value;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::iter::once;

#[derive(Clone, Debug)]
pub enum Op {
    Add,
    Div,
    Mul,
    Pow,
    Sub,
    Var(String),
    Val(Value),
    Brackets(Box<Expression>, Brackets),
}

impl Op {
    const PRIORITY: [Self; 5] = [Self::Sub, Self::Add, Self::Mul, Self::Pow, Self::Div];
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Add => f.write_str(SYMBOL_ADD),
            Self::Div => f.write_str(SYMBOL_DIV),
            Self::Mul => f.write_str(SYMBOL_MUL),
            Self::Pow => f.write_str(SYMBOL_POW),
            Self::Sub => f.write_str(SYMBOL_SUB),
            Self::Var(literal) => f.write_str(literal),
            Self::Val(value) => f.write_str(&value.to_string()),
            Self::Brackets(expr, brackets) => {
                let (lhs, rhs) = brackets.get_symbols();
                f.write_str(lhs)?;
                f.write_str(&expr.to_string())?;
                f.write_str(rhs)
            }
        }
    }
}

impl PartialEq for Op {
    fn eq(&self, other: &Self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match (self, other) {
            (Self::Sub, Self::Sub) => true,
            (Self::Add, Self::Add) => true,
            (Self::Sub, Self::Add) => true,
            (Self::Add, Self::Sub) => true,

            (Self::Mul, Self::Mul) => true,
            (Self::Div, Self::Div) => true,
            (Self::Div, Self::Mul) => true,
            (Self::Mul, Self::Div) => true,

            (Self::Pow, Self::Pow) => true,
            (Self::Val(..), Self::Val(..)) => true,
            (Self::Var(..), Self::Var(..)) => true,
            (Self::Val(..), Self::Var(..)) => true,
            (Self::Var(..), Self::Val(..)) => true,
            (Self::Brackets(..), Self::Brackets(..)) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Op {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let empty = Self::Brackets(Box::from(Expression::empty()), Brackets::Round);
        let priority = Self::PRIORITY
            .iter()
            .chain(once(&empty))
            .collect::<Vec<_>>();

        match (
            priority.iter().position(|x| *x == self),
            priority.iter().position(|x| *x == other),
        ) {
            (Some(priority), Some(other_priority)) => priority.partial_cmp(&other_priority),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::symbolic::brackets::Brackets;
    use crate::symbolic::expression::Expression;
    use crate::symbolic::op::Op;
    use crate::symbolic::value::Value::R;

    #[test]
    fn priority_eq() {
        assert_eq!(Op::Add, Op::Add);
        assert_eq!(Op::Mul, Op::Mul);
        assert_eq!(Op::Div, Op::Div);
        assert_eq!(Op::Pow, Op::Pow);
        assert_eq!(
            Op::Brackets(Box::from(Expression::empty()), Brackets::Round),
            Op::Brackets(Box::from(Expression::empty()), Brackets::Round)
        );
        assert_eq!(Op::Var(String::from("x")), Op::Var(String::from("x")));
        assert_eq!(Op::Val(R(0.0)), Op::Val(R(1.0)));
        assert_eq!(Op::Val(R(2.0)), Op::Var(String::from("x")));
        assert_eq!(Op::Var(String::from("x")), Op::Val(R(3.0)));
        assert_ne!(Op::Mul, Op::Add);
        assert_ne!(Op::Add, Op::Mul);
        assert_ne!(Op::Add, Op::Pow);
        assert_ne!(Op::Mul, Op::Pow);
    }

    #[test]
    fn priority_ord() {
        assert!(Op::Add < Op::Mul);
        assert!(Op::Mul < Op::Pow);
        assert!(Op::Pow < Op::Brackets(Box::new(Expression::empty()), Brackets::Round))
    }
}
