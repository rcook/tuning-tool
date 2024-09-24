use crate::args::Args;
use crate::dump_scala_file::dump_scala_file;
use crate::dump_sysex_file::dump_sysex_file;
use anyhow::{bail, Result};
use clap::Parser;
use std::ffi::OsStr;
use std::fs::read_dir;
use std::path::Path;

pub(crate) fn run() -> anyhow::Result<()> {
    fn dump(path: &Path) -> Result<()> {
        match path.extension().and_then(OsStr::to_str) {
            Some("scl") => dump_scala_file(&path),
            Some("syx") => dump_sysex_file(&path),
            _ => Ok(()),
        }
    }

    let args = Args::parse();
    if args.start_path.is_file() {
        dump(&args.start_path)?;
    } else if args.start_path.is_dir() {
        for e in read_dir(&args.start_path)? {
            let e = e?;
            let path = args.start_path.join(e.file_name());
            dump(&path)?;
        }
    } else {
        bail!(
            "Cannot determine what {start_path} is supposed to be",
            start_path = args.start_path.display()
        )
    }

    Ok(())
}
