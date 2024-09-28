use crate::bulk_tuning_dump_reply::BulkTuningDumpReply;
use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub(crate) fn dump_sysex_file(syx_path: &Path) -> Result<()> {
    let file = File::open(syx_path)?;
    let message = BulkTuningDumpReply::from_bytes(file.bytes())?;
    println!("{message:?}");
    Ok(())
}
