use crate::midi_tuning_bulk_dump_reply::MidiTuningBulkDumpReply;
use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[allow(unused)]
pub(crate) fn dump_sysex_file(syx_path: &Path) -> Result<()> {
    let file = File::open(syx_path)?;
    let message = MidiTuningBulkDumpReply::from_bytes(file.bytes())?;
    println!("{message:?}");
    Ok(())
}
