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

use crate::evaluation_strategy::Symbolic;
use crate::key_frequency_mapping::{compute_symbolic, KeyFrequencyMapping};
use crate::keyboard_mapping_source::KeyboardMappingSource;
use crate::scl_file::SclFile;
use crate::sympy::Sympy;
use crate::tuning_tool_args::DumpTuningTableFormat;
use anyhow::Result;
use std::fs::File;
use std::io::{stdout, Write};
use std::iter::zip;
use std::path::{Path, PathBuf};

pub(crate) fn dump_tuning_table(
    scl_path: &Path,
    keyboard_mapping_source: &KeyboardMappingSource,
    output_path: &Option<PathBuf>,
    format: DumpTuningTableFormat,
    sympy: bool,
) -> Result<()> {
    fn dump(
        out: &mut dyn Write,
        scl_path: &Path,
        keyboard_mapping_source: &KeyboardMappingSource,
        mappings: &Vec<KeyFrequencyMapping<Symbolic>>,
        format: DumpTuningTableFormat,
        sympy: &Option<Sympy>,
    ) -> Result<()> {
        match format {
            DumpTuningTableFormat::Brief => {
                for mapping in mappings {
                    writeln!(out, "{f}", f = mapping.frequency)?;
                }
            }
            DumpTuningTableFormat::Detailed => {
                writeln!(out, "# Scale file: {path}", path = scl_path.display())?;
                writeln!(out, "# {keyboard_mapping_source}")?;

                if let Some(sympy) = sympy {
                    let inputs = mappings
                        .iter()
                        .map(|m| m.frequency().to_string())
                        .collect::<Vec<_>>();
                    let exprs = sympy.simplify_vec(&inputs)?;
                    for (mapping, expr) in zip(mappings, exprs) {
                        writeln!(out, "{mapping:<95}  {expr}", mapping = mapping.to_string())?;
                    }
                } else {
                    for mapping in mappings {
                        writeln!(out, "{mapping}")?;
                    }
                }
            }
        }
        Ok(())
    }

    let sympy = match sympy {
        true => Some(Sympy::new()?),
        false => None,
    };

    let scl_file = SclFile::read(scl_path)?;
    let scale = scl_file.scale();
    let keyboard_mapping = keyboard_mapping_source.make_keyboard_mapping(scale)?;
    let mappings = compute_symbolic(scale, &keyboard_mapping)?;

    match output_path {
        Some(output_path) => dump(
            &mut File::create_new(output_path)?,
            scl_path,
            keyboard_mapping_source,
            &mappings,
            format,
            &sympy,
        )?,
        None => dump(
            &mut stdout(),
            scl_path,
            keyboard_mapping_source,
            &mappings,
            format,
            &sympy,
        )?,
    }

    Ok(())
}
