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
    use anyhow::Result;
    use clap::Parser;
    use std::ffi::OsStr;
    use std::fs::read_dir;
    use std::path::Path;

    fn test_dir(start_dir: &Path) -> Result<()> {
        let extension = Some(OsStr::new("scl"));
        for e in read_dir(start_dir)? {
            let e = e?;
            let path = start_dir.join(e.file_name());
            if path.extension() == extension {
                test_file(&path)?;
            }
        }
        Ok(())
    }

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
    test_dir(&args.start_dir)?;

    Ok(())
}
