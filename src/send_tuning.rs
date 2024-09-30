use crate::frequencies::calculate_frequencies;
use crate::hex_dump::to_hex_dump;
use crate::kbm_file::KbmFile;
use crate::note_change::NoteChange;
use crate::note_change_entry::NoteChangeEntry;
use crate::scl_file::SclFile;
use anyhow::Result;
use midly::live::{LiveEvent, SystemCommon};
use midly::num::u7;
use std::path::Path;

fn make_messages(device_id: u7, preset: u7, entries: &[NoteChangeEntry]) -> Result<Vec<Vec<u8>>> {
    let mut messages = Vec::new();
    for chunk in entries.chunks(64) {
        let note_change = NoteChange::new(device_id, preset, chunk)?;
        let vec = note_change.to_vec()?;
        let event = LiveEvent::Common(SystemCommon::SysEx(&vec));
        let mut buffer = Vec::new();
        event.write_std(&mut buffer)?;
        messages.push(buffer);
    }
    Ok(messages)
}

pub(crate) fn send_tuning(
    _midi_port_name: &str,
    scl_path: &Path,
    kbm_path: &Path,
    device_id: u7,
    preset: u7,
) -> Result<()> {
    let scl_file = SclFile::read(scl_path)?;
    let kbm_file = KbmFile::read(kbm_path)?;
    let entries = calculate_frequencies(scl_file.scale(), kbm_file.keyboard_mapping())
        .iter()
        .enumerate()
        .map(|(i, f)| {
            Ok(NoteChangeEntry {
                #[allow(clippy::unnecessary_fallible_conversions)]
                kk: TryInto::<u8>::try_into(i)?.try_into()?,
                mts: f.to_mts_entry(),
            })
        })
        .collect::<Result<Vec<_>>>()?;

    for message in make_messages(device_id, preset, &entries)? {
        println!("{}", to_hex_dump(&message, None)?);
    }
    Ok(())
}
