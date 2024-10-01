use crate::bulk_dump_reply::BulkDumpReply;
use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub(crate) fn dump_sysex_file(syx_path: &Path) -> Result<()> {
    let file = File::open(syx_path)?;
    let message = BulkDumpReply::from_bytes(file.bytes())?;
    println!("{message:?}");
    Ok(())
}
