use crate::bulk_dump_reply::BulkDumpReply;
use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub(crate) fn decode_bulk_dump(syx_path: &Path) -> Result<()> {
    let mut f = File::open(syx_path)?;
    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes)?;
    let bulk_dump_reply = BulkDumpReply::from_bytes(bytes.bytes())?;
    println!("Name: {name}", name = bulk_dump_reply.name());
    println!(
        "Device ID: {device_id}",
        device_id = bulk_dump_reply.device_id()
    );
    println!("Preset: {preset}", preset = bulk_dump_reply.preset());
    for (i, entry) in bulk_dump_reply.entries().iter().enumerate() {
        println!("{i:>3}: {frequency} Hz", frequency = entry.to_frequency().0);
    }
    Ok(())
}
