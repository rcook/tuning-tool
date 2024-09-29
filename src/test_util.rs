use crate::resources::RESOURCE_DIR;
use crate::scl_file::SclFile;
use anyhow::{anyhow, Result};
use std::path::Path;

#[allow(unused)]
pub(crate) fn read_test_syx_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    Ok(RESOURCE_DIR
        .get_file(path)
        .ok_or_else(|| anyhow!("Could not load tuning dump"))?
        .contents()
        .to_vec())
}

#[allow(unused)]
pub(crate) fn read_test_scl_file() -> Result<SclFile> {
    let file = RESOURCE_DIR
        .get_file("scl/carlos_super.scl")
        .ok_or_else(|| anyhow!("Could not get .scl file"))?;
    let s = file
        .contents_utf8()
        .ok_or_else(|| anyhow!("Could not convert to string"))?;
    s.parse::<SclFile>()
}
