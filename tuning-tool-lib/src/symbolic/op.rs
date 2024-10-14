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

#[derive(Clone, Debug)]
pub(crate) enum Op {
    Add,
    Div,
    Mul,
    Pow,
    Sub,
    Var,
    Val,
    Brackets,
}

impl Op {
    pub(crate) fn is_lower_precedence_than(&self, other: &Self) -> bool {
        fn precedence(op: &Op) -> Option<usize> {
            match op {
                Op::Add | Op::Sub => Some(0),
                Op::Div | Op::Mul => Some(1),
                Op::Pow => Some(3),
                Op::Brackets => Some(4),
                Op::Var | Op::Val => None,
            }
        }

        precedence(self)
            .and_then(|p0| precedence(other).map(|p1| p0 < p1))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use crate::symbolic::op::Op::{Add, Brackets, Div, Mul, Pow, Sub, Val, Var};

    #[test]
    fn precedence() {
        assert!(!Add.is_lower_precedence_than(&Add));
        assert!(Add.is_lower_precedence_than(&Div));
        assert!(Add.is_lower_precedence_than(&Mul));
        assert!(Add.is_lower_precedence_than(&Pow));
        assert!(!Add.is_lower_precedence_than(&Sub));
        assert!(!Add.is_lower_precedence_than(&Var));
        assert!(!Add.is_lower_precedence_than(&Val));
        assert!(Add.is_lower_precedence_than(&Brackets));
        assert!(!Div.is_lower_precedence_than(&Add));
        assert!(!Div.is_lower_precedence_than(&Div));
        assert!(!Div.is_lower_precedence_than(&Mul));
        assert!(Div.is_lower_precedence_than(&Pow));
        assert!(!Div.is_lower_precedence_than(&Sub));
        assert!(!Div.is_lower_precedence_than(&Var));
        assert!(!Div.is_lower_precedence_than(&Val));
        assert!(Div.is_lower_precedence_than(&Brackets));
        assert!(!Mul.is_lower_precedence_than(&Add));
        assert!(!Mul.is_lower_precedence_than(&Div));
        assert!(!Mul.is_lower_precedence_than(&Mul));
        assert!(Mul.is_lower_precedence_than(&Pow));
        assert!(!Mul.is_lower_precedence_than(&Sub));
        assert!(!Mul.is_lower_precedence_than(&Var));
        assert!(!Mul.is_lower_precedence_than(&Val));
        assert!(Mul.is_lower_precedence_than(&Brackets));
        assert!(!Pow.is_lower_precedence_than(&Add));
        assert!(!Pow.is_lower_precedence_than(&Div));
        assert!(!Pow.is_lower_precedence_than(&Mul));
        assert!(!Pow.is_lower_precedence_than(&Pow));
        assert!(!Pow.is_lower_precedence_than(&Sub));
        assert!(!Pow.is_lower_precedence_than(&Var));
        assert!(!Pow.is_lower_precedence_than(&Val));
        assert!(Pow.is_lower_precedence_than(&Brackets));
        assert!(!Sub.is_lower_precedence_than(&Add));
        assert!(Sub.is_lower_precedence_than(&Div));
        assert!(Sub.is_lower_precedence_than(&Mul));
        assert!(Sub.is_lower_precedence_than(&Pow));
        assert!(!Sub.is_lower_precedence_than(&Sub));
        assert!(!Sub.is_lower_precedence_than(&Var));
        assert!(!Sub.is_lower_precedence_than(&Val));
        assert!(Sub.is_lower_precedence_than(&Brackets));
        assert!(!Var.is_lower_precedence_than(&Add));
        assert!(!Var.is_lower_precedence_than(&Div));
        assert!(!Var.is_lower_precedence_than(&Mul));
        assert!(!Var.is_lower_precedence_than(&Pow));
        assert!(!Var.is_lower_precedence_than(&Sub));
        assert!(!Var.is_lower_precedence_than(&Var));
        assert!(!Var.is_lower_precedence_than(&Val));
        assert!(!Var.is_lower_precedence_than(&Brackets));
        assert!(!Val.is_lower_precedence_than(&Add));
        assert!(!Val.is_lower_precedence_than(&Div));
        assert!(!Val.is_lower_precedence_than(&Mul));
        assert!(!Val.is_lower_precedence_than(&Pow));
        assert!(!Val.is_lower_precedence_than(&Sub));
        assert!(!Val.is_lower_precedence_than(&Var));
        assert!(!Val.is_lower_precedence_than(&Val));
        assert!(!Val.is_lower_precedence_than(&Brackets));
        assert!(!Brackets.is_lower_precedence_than(&Add));
        assert!(!Brackets.is_lower_precedence_than(&Div));
        assert!(!Brackets.is_lower_precedence_than(&Mul));
        assert!(!Brackets.is_lower_precedence_than(&Pow));
        assert!(!Brackets.is_lower_precedence_than(&Sub));
        assert!(!Brackets.is_lower_precedence_than(&Var));
        assert!(!Brackets.is_lower_precedence_than(&Val));
        assert!(!Brackets.is_lower_precedence_than(&Brackets));
    }
}
