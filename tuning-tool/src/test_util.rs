#![cfg(test)]

use crate::resources::RESOURCE_DIR;
use crate::scl_file::SclFile;
use anyhow::{anyhow, Result};
use std::path::Path;

#[allow(unused)]
pub(crate) fn read_expected_frequencies<P: AsRef<Path>>(path: P) -> Result<Vec<f64>> {
    let path = path.as_ref();
    let file = RESOURCE_DIR.get_file(path).ok_or_else(|| {
        anyhow!(
            "Expected frequency file {path} could not be opened",
            path = path.display()
        )
    })?;
    let s = file.contents_utf8().ok_or_else(|| {
        anyhow!(
            "Could not decode frequency file {path} as UTF-8",
            path = path.display()
        )
    })?;

    s.lines()
        .filter_map(|line| {
            let temp = line.trim();
            if temp.is_empty() {
                None
            } else {
                Some(temp)
            }
        })
        .map(|line| line.parse::<f64>().map_err(|e| anyhow!(e)))
        .collect::<Result<Vec<_>>>()
}

#[allow(unused)]
pub(crate) fn read_test_syx_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let path = path.as_ref();
    Ok(RESOURCE_DIR
        .get_file(path)
        .ok_or_else(|| anyhow!("Could not load tuning dump {path}", path = path.display()))?
        .contents()
        .to_vec())
}

#[allow(unused)]
pub(crate) fn read_test_scl_file<P: AsRef<Path>>(path: P) -> Result<SclFile> {
    let path = path.as_ref();
    let file = RESOURCE_DIR
        .get_file(path)
        .ok_or_else(|| anyhow!("Could not load .scl file {path}", path = path.display()))?;
    let s = file.contents_utf8().ok_or_else(|| {
        anyhow!(
            "Could not convert contents of {path} to string",
            path = path.display()
        )
    })?;
    s.parse::<SclFile>()
}
