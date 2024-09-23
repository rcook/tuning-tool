mod args;
mod cli;
mod dump_scala_file;
mod dump_sysex_file;
mod fs;
mod midi_tuning_bulk_dump_reply;
mod note;
mod notes;
mod scala;
mod tuning;

fn main() -> anyhow::Result<()> {
    use crate::args::Args;
    use crate::dump_sysex_file::dump_sysex_file;
    use anyhow::bail;
    use clap::Parser;
    use std::ffi::OsStr;
    use std::fs::read_dir;

    let args = Args::parse();
    if args.start_path.is_file() {
        dump_sysex_file(&args.start_path)?;
    } else if args.start_path.is_dir() {
        let extension = Some(OsStr::new("scl"));
        for e in read_dir(&args.start_path)? {
            let e = e?;
            let path = args.start_path.join(e.file_name());
            if path.extension() == extension {
                dump_sysex_file(&path)?;
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
