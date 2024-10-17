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

use crate::evaluate::Evaluate;
use crate::frequency::Frequency;
use crate::interval::Interval;
use crate::ratio::Ratio;
use crate::scale::Scale;
use num::pow::Pow;
use std::fmt::Display;
use std::ops::{Div, Mul};
use tuning_tool_lib::symbolic::Expression;

pub(crate) trait EvaluationStrategy {
    type Frequency: Clone
        + Display
        + Div<Self::Ratio, Output = Self::Frequency>
        + Evaluate
        + Mul<Self::Ratio, Output = Self::Frequency>;
    type Ratio: Clone + Evaluate + Pow<i32, Output = Self::Ratio>;

    fn new_frequency(value: f64) -> Self::Frequency;
    fn equave_ratio(scale: &Scale) -> Self::Ratio;
    fn interval_ratio(interval: &Interval) -> Self::Ratio;
}

pub(crate) struct Symbolic;

impl EvaluationStrategy for Symbolic {
    type Frequency = Expression;
    type Ratio = Expression;

    fn new_frequency(value: f64) -> Self::Frequency {
        Expression::new_r(value)
    }

    fn equave_ratio(scale: &Scale) -> Self::Ratio {
        scale.equave_ratio_expr()
    }

    fn interval_ratio(interval: &Interval) -> Self::Ratio {
        interval.as_ratio_expr()
    }
}

pub(crate) struct Direct;

impl EvaluationStrategy for Direct {
    type Frequency = Frequency;
    type Ratio = Ratio;

    fn new_frequency(value: f64) -> Self::Frequency {
        Frequency(value)
    }

    fn equave_ratio(scale: &Scale) -> Self::Ratio {
        scale.equave_ratio()
    }

    fn interval_ratio(interval: &Interval) -> Self::Ratio {
        interval.as_ratio()
    }
}
