use crate::tuning::Tuning;
use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub(crate) fn read_to_string_lossy(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = vec![];
    file.read_to_end(&mut buffer)?;
    Ok(String::from_utf8_lossy(&buffer).to_string())
}

pub(crate) fn read_scala_file(scl_path: &Path) -> Result<Tuning> {
    read_to_string_lossy(scl_path)?.parse()
}
