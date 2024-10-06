use crate::devices::{get_midi_output_port, make_midi_output};
use crate::frequencies::calculate_frequencies;
use crate::frequency::Frequency;
use crate::hex_dump::to_hex_dump;
use crate::kbm_file::KbmFile;
use crate::note_change::NoteChange;
use crate::note_change_entry::NoteChangeEntry;
use crate::scl_file::SclFile;
use crate::tuning_tool_args::SendTuningOutput;
use crate::types::{ChunkSize, DeviceId, MidiValue, Preset};
use anyhow::Result;
use midly::live::{LiveEvent, SystemCommon};
use midly::num::u7;
use std::fs::File;
use std::io::Write;
use std::iter::zip;
use std::path::Path;

fn make_note_change_entries(
    scl_file: &SclFile,
    kbm_file: &KbmFile,
) -> Result<Vec<(Frequency, NoteChangeEntry)>> {
    calculate_frequencies(scl_file.scale(), kbm_file.keyboard_mapping())
        .iter()
        .enumerate()
        .map(|(i, f)| {
            Ok((
                *f,
                NoteChangeEntry {
                    #[allow(clippy::unnecessary_fallible_conversions)]
                    key_number: TryInto::<u8>::try_into(i)?.try_into()?,
                    mts: f.to_mts_entry()?,
                },
            ))
        })
        .collect::<Result<Vec<_>>>()
}

fn make_messages(
    device_id: DeviceId,
    preset: Preset,
    entries: &[NoteChangeEntry],
    chunk_size: ChunkSize,
) -> Result<Vec<Vec<u8>>> {
    let mut messages = Vec::new();
    for chunk in entries.chunks(chunk_size.to_u8() as usize) {
        let note_change = NoteChange::new(device_id, preset, chunk)?;
        let vec = note_change.to_vec()?;
        let u7_slice = u7::slice_from_int(MidiValue::to_u8_slice(&vec));
        let event = LiveEvent::Common(SystemCommon::SysEx(u7_slice));
        let mut buffer = Vec::new();
        event.write_std(&mut buffer)?;
        messages.push(buffer);
    }
    Ok(messages)
}

pub(crate) fn send_tuning(
    scl_path: &Path,
    kbm_path: &Path,
    output: &SendTuningOutput,
    device_id: DeviceId,
    preset: Preset,
    chunk_size: ChunkSize,
) -> Result<()> {
    let scl_file = SclFile::read(scl_path)?;
    let kbm_file = KbmFile::read(kbm_path)?;

    let keyboard_mapping = kbm_file.keyboard_mapping();
    println!(
        "Start MIDI note: {value} (0x{value:02x})",
        value = keyboard_mapping.start_note_number()
    );
    println!(
        "End MIDI note: {value} (0x{value:02x})",
        value = keyboard_mapping.end_note_number()
    );
    println!(
        "Base MIDI note: {value} (0x{value:02x})",
        value = keyboard_mapping.base_note_number()
    );
    println!(
        "Base frequency: {value} Hz",
        value = keyboard_mapping.base_frequency()
    );

    let data = make_note_change_entries(&scl_file, &kbm_file)?;
    let frequencies = data.iter().map(|x| x.0).collect::<Vec<_>>();
    let entries = data.into_iter().map(|x| x.1).collect::<Vec<_>>();

    let messages = make_messages(device_id, preset, &entries, chunk_size)?;

    match (&output.output_port, &output.syx_path) {
        (Some(output_port), None) => {
            let midi_output = make_midi_output()?;
            let midi_output_port = get_midi_output_port(&midi_output, output_port)?;
            let mut conn = midi_output.connect(&midi_output_port, "tuning-tool")?;
            for message in messages {
                println!("{}", to_hex_dump(&message, None)?);
                conn.send(&message)?;
            }
        }
        (None, Some(syx_path)) => {
            let mut f = File::create_new(syx_path)?;
            for message in messages {
                f.write_all(&message)?;
            }
        }
        (None, None) => {
            for (i, (message, frequency)) in zip(messages, frequencies).enumerate() {
                println!(
                    "{i:>3}: {hex} ({frequency:.1} Hz)",
                    hex = to_hex_dump(&message, None)?,
                    frequency = frequency.0
                );
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}
