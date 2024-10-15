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

use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    R(f64),
    Z(i32),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::R(value) => f.write_str(&value.to_string()),
            Self::Z(value) => f.write_str(&value.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::symbolic::value::Value::{R, Z};

    #[test]
    fn basics() {
        assert_eq!("5", R(5f64).to_string());
        assert_eq!("5.1", R(5.1f64).to_string());
        assert_eq!("5", Z(5).to_string());
    }
}
