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

use crate::devices::{get_midi_output_port, make_midi_output};
use crate::frequency::Frequency;
use crate::hex_dump::to_hex_dump;
use crate::key_frequency_mapping::compute_direct;
use crate::keyboard_mapping::KeyboardMapping;
use crate::keyboard_mapping_source::KeyboardMappingSource;
use crate::midi_output_ex::MidiOutputEx;
use crate::note_change::NoteChange;
use crate::note_change_entry::NoteChangeEntry;
use crate::scale::Scale;
use crate::scl_file::SclFile;
use crate::send_tuning_output::SendTuningOutput;
use crate::types::{ChunkSize, DeviceId, MidiValue, Preset};
use anyhow::Result;
use midly::live::{LiveEvent, SystemCommon};
use midly::num::u7;
use std::fs::File;
use std::io::Write;
use std::iter::zip;
use std::path::Path;

fn make_note_change_entries(
    scale: &Scale,
    keyboard_mapping: &KeyboardMapping,
) -> Result<Vec<(Frequency, NoteChangeEntry)>> {
    compute_direct(scale, keyboard_mapping)?
        .iter()
        .enumerate()
        .map(|(i, mapping)| {
            let f = Frequency(mapping.frequency.0);
            Ok((
                f,
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
    keyboard_mapping_source: &KeyboardMappingSource,
    output: &SendTuningOutput,
    device_id: DeviceId,
    preset: Preset,
    chunk_size: ChunkSize,
) -> Result<()> {
    let scl_file = SclFile::read(scl_path)?;
    let scale = scl_file.scale();
    let keyboard_mapping = keyboard_mapping_source.make_keyboard_mapping(scale)?;
    println!(
        "Start MIDI note: {value} (0x{value:02x})",
        value = keyboard_mapping.start_key()
    );
    println!(
        "End MIDI note: {value} (0x{value:02x})",
        value = keyboard_mapping.end_key()
    );
    println!(
        "Base MIDI note: {value} (0x{value:02x})",
        value = keyboard_mapping.reference_key()
    );
    println!(
        "Base frequency: {value} Hz",
        value = keyboard_mapping.reference_frequency()
    );

    let data = make_note_change_entries(scale, &keyboard_mapping)?;
    let frequencies = data.iter().map(|x| x.0).collect::<Vec<_>>();
    let entries = data.into_iter().map(|x| x.1).collect::<Vec<_>>();

    let messages = make_messages(device_id, preset, &entries, chunk_size)?;

    match output {
        SendTuningOutput::OutputPort(output_port) => {
            let midi_output = make_midi_output()?;
            let midi_output_port = get_midi_output_port(&midi_output, output_port)?;
            let mut conn = midi_output.connect_ex(&midi_output_port, "tuning-tool")?;
            for message in messages {
                println!("{}", to_hex_dump(&message, None)?);
                conn.send(&message)?;
            }
        }
        SendTuningOutput::SyxPath(syx_path) => {
            let mut f = File::create_new(syx_path)?;
            for message in messages {
                f.write_all(&message)?;
            }
        }
        SendTuningOutput::Stdout => {
            for (i, (message, frequency)) in zip(messages, frequencies).enumerate() {
                println!(
                    "(MIDI message {i}): {hex} ({frequency:.1} Hz)",
                    hex = to_hex_dump(&message, None)?,
                    frequency = frequency.0
                );
            }
        }
    }

    Ok(())
}
