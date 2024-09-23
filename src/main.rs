mod args;
mod cli;
mod fs;
mod note;
mod scala;
mod scale;

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
        let scale = read_scala_file(scl_path)?;
        for (i, note) in scale.notes().into_iter().enumerate() {
            match note.cents() {
                Some(cents) => println!("(step {i}): {cents}"),
                None => println!("(step {i}): (could not calculate cents)"),
            }
        }
        Ok(())
    }

    let args = Args::parse();
    test_dir(&args.start_dir)?;

    Ok(())
}
