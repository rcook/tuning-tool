use crate::frequencies::calculate_frequencies;
use crate::hex_dump::to_hex_dump;
use crate::kbm_file::KbmFile;
use crate::note_change::NoteChange;
use crate::note_change_entry::NoteChangeEntry;
use crate::scl_file::SclFile;
use anyhow::{bail, Result};
use midir::{MidiOutput, MidiOutputPort};
use midly::live::{LiveEvent, SystemCommon};
use midly::num::u7;
use std::path::Path;

fn get_midi_output_port(midi_output: &MidiOutput, name: &str) -> Result<MidiOutputPort> {
    let mut names = Vec::new();
    for p in midi_output.ports() {
        let temp = midi_output.port_name(&p)?;
        if midi_output.port_name(&p)? == name {
            return Ok(p);
        }
        names.push(temp);
    }

    let s = names.join(", ");
    bail!("No MIDI output port with name {name} found: choose from {s}");
}

fn make_note_change_entries(
    scl_file: &SclFile,
    kbm_file: &KbmFile,
) -> Result<Vec<NoteChangeEntry>> {
    calculate_frequencies(scl_file.scale(), kbm_file.keyboard_mapping())
        .iter()
        .enumerate()
        .map(|(i, f)| {
            Ok(NoteChangeEntry {
                #[allow(clippy::unnecessary_fallible_conversions)]
                kk: TryInto::<u8>::try_into(i)?.try_into()?,
                mts: f.to_mts_entry()?,
            })
        })
        .collect::<Result<Vec<_>>>()
}

fn make_messages(
    device_id: u7,
    preset: u7,
    entries: &[NoteChangeEntry],
    chunk_size: u7,
) -> Result<Vec<Vec<u8>>> {
    let mut messages = Vec::new();
    for chunk in entries.chunks(chunk_size.as_int() as usize) {
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
    scl_path: &Path,
    kbm_path: &Path,
    midi_output_port_name: &Option<String>,
    device_id: u7,
    preset: u7,
    chunk_size: u7,
) -> Result<()> {
    let scl_file = SclFile::read(scl_path)?;
    let kbm_file = KbmFile::read(kbm_path)?;
    let entries = make_note_change_entries(&scl_file, &kbm_file)?;
    let messages = make_messages(device_id, preset, &entries, chunk_size)?;

    if let Some(midi_output_port_name) = midi_output_port_name {
        let midi_output = MidiOutput::new("MIDI output")?;
        let midi_output_port = get_midi_output_port(&midi_output, midi_output_port_name)?;
        let mut conn = midi_output.connect(&midi_output_port, "tuning-tool")?;
        for message in messages {
            println!("{}", to_hex_dump(&message, None)?);
            conn.send(&message)?;
        }
    } else {
        for message in messages {
            println!("{}", to_hex_dump(&message, None)?);
        }
    }

    Ok(())
}
