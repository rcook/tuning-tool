mod args;
mod note;
mod scale;

fn main() -> anyhow::Result<()> {
    use crate::args::Args;
    use crate::scale::Scale;
    use anyhow::Result;
    use clap::Parser;
    use std::ffi::OsStr;
    use std::fs::{read_dir, File};
    use std::io::Read;
    use std::path::Path;

    fn read_to_string_lossy(path: &std::path::Path) -> anyhow::Result<String> {
        let mut file = File::open(path)?;
        let mut buffer = vec![];
        file.read_to_end(&mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer).to_string())
    }

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
        let s = read_to_string_lossy(scl_path)?;
        let scale = s.parse::<Scale>()?;
        for (i, note) in scale.notes().into_iter().enumerate() {
            println!("(step {i}): {cents}", cents = note.cents());
        }
        Ok(())
    }

    let args = Args::parse();
    test_dir(&args.start_dir)?;

    Ok(())
}
