use crate::kbm_file::KbmFile;
use crate::key_frequency_mapping::KeyFrequencyMapping;
use crate::scl_file::SclFile;
use anyhow::Result;
use std::fs::File;
use std::io::{stdout, Write};
use std::path::{Path, PathBuf};

pub(crate) fn dump_tuning_table(
    scl_path: &Path,
    kbm_path: &Path,
    output_path: &Option<PathBuf>,
    brief: bool,
) -> Result<()> {
    fn dump(
        out: &mut dyn Write,
        scl_path: &Path,
        kbm_path: &Path,
        mappings: &Vec<KeyFrequencyMapping>,
        brief: bool,
    ) -> Result<()> {
        if brief {
            for mapping in mappings {
                writeln!(out, "{f}", f = mapping.frequency)?;
            }
        } else {
            writeln!(out, "# Scale file: {path}", path = scl_path.display())?;
            writeln!(
                out,
                "# Keyboard mapping file: {path}",
                path = kbm_path.display()
            )?;
            for mapping in mappings {
                writeln!(out, "{mapping}")?;
            }
        }
        Ok(())
    }

    let scl_file = SclFile::read(scl_path)?;
    let kbm_file = KbmFile::read(kbm_path)?;
    let scale = scl_file.scale();
    let keyboard_mapping = kbm_file.keyboard_mapping();
    let mappings = KeyFrequencyMapping::compute(scale, keyboard_mapping)?;

    match output_path {
        Some(output_path) => dump(
            &mut File::create_new(output_path)?,
            scl_path,
            kbm_path,
            &mappings,
            brief,
        )?,
        None => dump(&mut stdout(), scl_path, kbm_path, &mappings, brief)?,
    }

    Ok(())
}
