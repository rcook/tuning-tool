use crate::args::Args;
use crate::bulk_dump_reply::BulkDumpReply;
use crate::consts::{U7_MAX, U7_ZERO};
use crate::dump_sysex_file::dump_sysex_file;
use crate::frequencies::calculate_frequencies;
use crate::frequency::Frequency;
use crate::hex_dump::to_hex_dump;
use crate::keyboard_mapping::KeyboardMapping;
use crate::midi_note::MidiNote;
use crate::note_number::NoteNumber;
use crate::resources::RESOURCE_DIR;
use crate::scl_file::SclFile;
use crate::sysex_event::SysExEvent;
use anyhow::{anyhow, bail, Result};
use clap::Parser;
use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};
use midly::num::u7;
use midly::Smf;
use std::ffi::OsStr;
use std::fs::read_dir;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

#[allow(unused)]
pub(crate) fn show_all_midi_notes() {
    for midi_note in MidiNote::ALL {
        println!(
            "{note_number}: {name} ({frequency:.1} Hz)",
            note_number = midi_note.note_number().0,
            name = midi_note.name(),
            frequency = midi_note.frequency().0
        );
    }
}

#[allow(unused)]
pub(crate) fn nearest_below_or_equal() {
    let (midi_note, rem) =
        MidiNote::nearest_below_or_equal(Frequency(880f64)).expect("Must succeed");
    println!("{name} + {rem} Hz", name = midi_note.name(), rem = rem.0);
}

#[allow(unused)]
pub(crate) fn decode_sysex_events() -> Result<()> {
    let mid_dir = RESOURCE_DIR
        .get_dir("mid")
        .ok_or_else(|| anyhow!("Could not get mid directory"))?;
    for f in mid_dir.files() {
        println!("{}:", f.path().display());
        let bytes = f.contents();
        let smf = Smf::parse(bytes)?;
        for event in SysExEvent::find_all(&smf) {
            event.dump();
            event.decode();
        }
    }
    Ok(())
}

#[allow(unused)]
pub(crate) fn cli(start_path: &Path) -> Result<()> {
    fn dump(path: &Path) -> Result<()> {
        match path.extension().and_then(OsStr::to_str) {
            Some("scl") => {
                SclFile::read(path)?.dump();
                Ok(())
            }
            Some("syx") => dump_sysex_file(path),
            _ => Ok(()),
        }
    }

    let args = Args::parse();
    if start_path.is_file() {
        dump(start_path)?;
    } else if start_path.is_dir() {
        for e in read_dir(start_path)? {
            let e = e?;
            let path = start_path.join(e.file_name());
            dump(&path)?;
        }
    } else {
        bail!(
            "Cannot determine what {start_path} is supposed to be",
            start_path = start_path.display()
        )
    }
    Ok(())
}

#[allow(unused)]
pub(crate) fn generate_message() -> Result<()> {
    // https://forums.steinberg.net/t/microtonal-midi-messages-vst-3/831268/9
    fn show_message(device_id: u8, program_number: u8, kk: u8, target_midi: f64) -> Result<()> {
        let xx = target_midi as i32; // i.e. 69
        let semitones = target_midi - xx as f64; // i.e. 0.33
        let semitones_14bit = (semitones * (0x4000 as f64)) as u16; // i.e. 5406
        let yy = semitones_14bit / 0x80; // i.e. 42
        let zz = semitones_14bit - 0x80 * yy; // i.e. 30

        // Single Note Tuning Change Real-Time message
        const LL: u8 = 1;
        let length = 8 + LL * 4;
        let mut data = Vec::with_capacity(length as usize);

        // Universal Real-Time SysEx header
        data.push(0xF0);
        data.push(0x7f);

        // Device ID
        data.push(device_id);

        // MIDI Tuning
        data.push(0x08);

        // Note Change
        data.push(0x02);

        // Tuning program number
        data.push(program_number);

        // Number of changes
        data.push(LL);

        // MIDI key number
        data.push(kk);

        // Frequency data
        data.push(xx as u8);
        data.push(yy as u8);
        data.push(zz as u8);

        // End
        data.push(0xf7);

        assert!(data.len() == length as usize);

        println!("{}", to_hex_dump(&data, None)?);

        Ok(())
    }

    show_message(0, 0, 0x30, 69.33f64)?; // just sharp of A4
    Ok(())
}

#[allow(unused)]
pub(crate) fn enumerate_midi_outputs() -> Result<()> {
    let midi_output = MidiOutput::new("MIDI output")?;
    if midi_output.port_count() == 0 {
        println!("You have no MIDI output ports");
    } else {
        for p in midi_output.ports() {
            let name = midi_output.port_name(&p)?;
            println!("MIDI output port: {name}");
        }
    }
    Ok(())
}

#[allow(unused)]
pub(crate) fn play_note() -> Result<()> {
    fn get_output_port(midi_output: &MidiOutput, name: &str) -> Result<Option<MidiOutputPort>> {
        for p in midi_output.ports() {
            if midi_output.port_name(&p)? == name {
                return Ok(Some(p));
            }
        }
        Ok(None)
    }

    fn play_note(
        conn: &mut MidiOutputConnection,
        note_number: u8,
        duration: Duration,
    ) -> Result<()> {
        const NOTE_ON: u8 = 0x90;
        const NOTE_OFF: u8 = 0x80;
        const VELOCITY: u8 = 0x64;
        conn.send(&[NOTE_ON, note_number, VELOCITY])?;
        sleep(duration);
        conn.send(&[NOTE_OFF, note_number, VELOCITY])?;
        Ok(())
    }

    let midi_output = MidiOutput::new("MIDI output")?;
    let Some(port) = get_output_port(&midi_output, "MidiView")? else {
        bail!("Couldn't find port with specified name")
    };

    let mut conn = midi_output.connect(&port, "tuning-tool")?;
    play_note(&mut conn, 60, Duration::from_millis(1000))?;

    Ok(())
}

#[allow(unused)]
pub(crate) fn send_tuning_sysex() -> Result<()> {
    fn get_output_port(midi_output: &MidiOutput, name: &str) -> Result<Option<MidiOutputPort>> {
        for p in midi_output.ports() {
            if midi_output.port_name(&p)? == name {
                return Ok(Some(p));
            }
        }
        Ok(None)
    }

    let scl_file = RESOURCE_DIR
        .get_file("scl/carlos_super.scl")
        .ok_or_else(|| anyhow!("Could not get scl file"))?;
    let s = scl_file
        .contents_utf8()
        .ok_or_else(|| anyhow!("Could not convert to string"))?;
    let scala_file = s.parse::<SclFile>()?;

    let scale = scala_file.scale();

    let keyboard_mapping = KeyboardMapping::new(
        NoteNumber(U7_ZERO),
        NoteNumber(U7_MAX),
        NoteNumber::ZERO,
        Frequency::MIDI_MIN,
    )?;
    let entries = calculate_frequencies(scale, &keyboard_mapping)
        .iter()
        .map(|f| f.to_mts_entry())
        .collect::<Vec<_>>();
    let entries = entries
        .try_into()
        .expect("Must contain exactly 128 elements");
    let reply = BulkDumpReply::new(
        U7_ZERO,
        u7::from_int_lossy(8),
        "carlos_super.mid".parse()?,
        entries,
    )?;
    let bytes = reply.to_bytes_with_start_and_end()?;

    let midi_output = MidiOutput::new("MIDI output")?;
    let Some(port) = get_output_port(&midi_output, "MidiView")? else {
        bail!("Couldn't find port with specified name")
    };

    let mut conn = midi_output.connect(&port, "test")?;
    println!("Sending {} bytes", bytes.len());
    conn.send(&bytes)?;
    Ok(())
}
