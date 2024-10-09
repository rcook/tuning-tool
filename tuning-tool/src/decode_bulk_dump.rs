// Copyright (c) 2024 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

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
