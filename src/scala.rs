use crate::fs::read_to_string_lossy;
use crate::scale::Scale;
use anyhow::Result;
use std::path::Path;

pub(crate) fn read_scala_file(scl_path: &Path) -> Result<Scale> {
    Ok(read_to_string_lossy(scl_path)?.parse()?)
}
