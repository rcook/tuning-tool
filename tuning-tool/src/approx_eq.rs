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

#![cfg(test)]

macro_rules! assert_approx_eq {
    ($left: expr, $right: expr) => {
        let (lhs, rhs): (f64, f64) = ($left, $right);
        assert!(
            lhs.approx_eq(rhs),
            "left={lhs} and right={rhs} are not approximately equal (default epsilon)"
        );
    };
    ($left: expr, $right: expr, $epsilon: expr) => {
        let (lhs, rhs, ep): (f64, f64, f64) = ($left, $right, $epsilon);
        assert!(
            lhs.approx_eq_with_epsilon(rhs, ep),
            "left={lhs} and right={rhs} are not approximately equal (epsilon={ep})"
        );
    };
}
pub(crate) use assert_approx_eq;

pub(crate) trait ApproxEq {
    fn approx_eq(&self, other: Self) -> bool;
    fn approx_eq_with_epsilon(&self, other: Self, epsilon: Self) -> bool;
}

impl ApproxEq for f64 {
    fn approx_eq(&self, other: Self) -> bool {
        self.approx_eq_with_epsilon(other, Self::EPSILON)
    }

    fn approx_eq_with_epsilon(&self, other: Self, epsilon: Self) -> bool {
        (self - other).abs() < epsilon
    }
}

#[cfg(test)]
mod tests {
    use crate::approx_eq::ApproxEq;

    #[test]
    fn basics() {
        assert!(0f64.approx_eq(0f64));
        assert!(!0.0001f64.approx_eq(0f64));
        assert!(!0.0001f64.approx_eq_with_epsilon(0f64, 0.00001f64));
        assert!(!0.0001f64.approx_eq_with_epsilon(0f64, 0.0001f64));
        assert!(0.0001f64.approx_eq_with_epsilon(0f64, 0.001f64));
        assert!(0.0001f64.approx_eq_with_epsilon(0f64, 0.01f64));
        assert!(0.0001f64.approx_eq_with_epsilon(0f64, 0.1f64));
    }
}
