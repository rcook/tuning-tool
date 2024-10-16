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

use anyhow::{bail, Result};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use tempfile::NamedTempFile;
use which::which;

pub(crate) struct Simplifier {
    python: Python,
}

impl Simplifier {
    pub(crate) fn new() -> Result<Self> {
        let python = Python::new()?;

        if python.exec("import sympy").is_err() {
            bail!("sympy not installed");
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

struct Python {
    python_path: PathBuf,
}

impl Python {
    fn new() -> Result<Self> {
        let python_path = which("python3")?;
        Ok(Self { python_path })
    }

    fn exec(&self, s: &str) -> Result<String> {
        let mut file = NamedTempFile::with_suffix(".py")?;
        writeln!(file, "{}", s)?;
        let path = file.into_temp_path();

        let output = Command::new(&self.python_path).arg(&path).output()?;
        if !output.status.success() {
            bail!("Python script failed");
        }

        Ok(String::from_utf8(output.stdout)?)
    }
}
