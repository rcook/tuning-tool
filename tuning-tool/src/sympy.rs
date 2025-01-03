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

use crate::python::Python;
use anyhow::{bail, Result};

pub(crate) struct Sympy {
    python: Python,
}

impl Sympy {
    pub(crate) fn new() -> Result<Self> {
        let python = Python::new(&None)?;

        if python.exec("import sympy").is_err() {
            bail!(
                "sympy not installed for Python at {python_path}",
                python_path = python.python_path().display()
            );
        }

        Ok(Self { python })
    }

    #[allow(unused)]
    fn simplify(&self, s: &str) -> Result<String> {
        let Ok(stdout) = self.python.exec(&format!(
            "from sympy import simplify\nprint(simplify(\"{s}\"))"
        )) else {
            bail!("Simplify failed");
        };

        Ok(String::from(stdout.trim()))
    }

    pub(crate) fn simplify_vec(&self, inputs: &Vec<String>) -> Result<Vec<String>> {
        let mut script = String::from("from sympy import simplify\n");
        for input in inputs {
            script.push_str(&format!("print(simplify(\"{input}\"))\n"));
        }
        let Ok(stdout) = self.python.exec(&script) else {
            bail!("Simplify failed");
        };

        let lines = stdout.trim().lines().map(String::from).collect::<Vec<_>>();

        if lines.len() != inputs.len() {
            bail!("Output was invalid")
        }

        Ok(lines)
    }
}
