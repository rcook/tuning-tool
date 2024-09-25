use crate::fs::read_to_string_lossy;
use crate::scala::tuning::Tuning;
use anyhow::Result;
use std::path::Path;

pub(crate) fn read_scala_file(scl_path: &Path) -> Result<Tuning> {
    read_to_string_lossy(scl_path)?.parse()
}
