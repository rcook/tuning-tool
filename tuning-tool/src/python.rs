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
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::NamedTempFile;
use which::which;

const PYTHON_FILE_NAME: &str = "python3";

pub(crate) struct Python {
    python_path: PathBuf,
}

impl Python {
    pub(crate) fn new(python_file_name: &Option<String>) -> Result<Self> {
        let python_file_name = python_file_name.as_deref().unwrap_or(PYTHON_FILE_NAME);
        let Ok(python_path) = which(python_file_name) else {
            bail!("Cannot find Python binary {python_file_name}");
        };
        Ok(Self { python_path })
    }

    pub(crate) fn python_path(&self) -> &Path {
        &self.python_path
    }

    pub(crate) fn exec(&self, s: &str) -> Result<String> {
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
