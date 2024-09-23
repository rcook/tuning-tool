mod args;
mod cli;
mod fs;
mod note;
mod notes;
mod scala;
mod tuning;

fn main() -> anyhow::Result<()> {
    use crate::args::Args;
    use crate::scala::read_scala_file;
    use anyhow::{bail, Result};
    use clap::Parser;
    use std::ffi::OsStr;
    use std::fs::read_dir;
    use std::path::Path;

    fn test_file(scl_path: &Path) -> Result<()> {
        println!("Testing {}", scl_path.display());
        let tuning = read_scala_file(scl_path)?;

        if let Some(file_name) = tuning.file_name() {
            println!("File name: {file_name}");
        }

        println!(
            "Description: {description}",
            description = tuning.description()
        );

        println!("Steps: {step_count}", step_count = tuning.step_count());
        println!("Notes: {note_count}", note_count = tuning.note_count());

        for (i, note) in tuning.notes().enumerate() {
            match note.cents() {
                Some(cents) => println!("(note {i}): {cents}"),
                None => println!("(note {i}): (could not calculate cents)"),
            }
        }
        Ok(())
    }

    let args = Args::parse();
    if args.start_path.is_file() {
        test_file(&args.start_path)?;
    } else if args.start_path.is_dir() {
        let extension = Some(OsStr::new("scl"));
        for e in read_dir(&args.start_path)? {
            let e = e?;
            let path = args.start_path.join(e.file_name());
            if path.extension() == extension {
                test_file(&path)?;
            }
        }
    } else {
        bail!(
            "Cannot determine what {start_path} is supposed to be",
            start_path = args.start_path.display()
        )
    }

    Ok(())
}
