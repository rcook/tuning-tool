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

use crate::interval::Interval;
use anyhow::{bail, Result};
use tuning_tool_lib::symbolic::Expression;

#[derive(Debug)]
pub(crate) struct Scale {
    intervals: Vec<Interval>,
}

impl Scale {
    pub(crate) fn new(intervals: Vec<Interval>) -> Result<Self> {
        if intervals.is_empty() {
            bail!("Need at least one interval");
        }
        Ok(Self { intervals })
    }

    pub(crate) fn intervals(&self) -> &Vec<Interval> {
        &self.intervals
    }

    pub(crate) fn equave_ratio(&self) -> Expression {
        let last_interval = self
            .intervals
            .last()
            .expect("Must have at least one interval");
        last_interval.as_ratio()
    }
}
