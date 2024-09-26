use crate::args::Args;
use crate::consts::{BASE_FREQUENCY, BASE_MIDI_NOTE};
use crate::dump_scala_file::dump_scala_file;
use crate::dump_sysex_file::dump_sysex_file;
use crate::midi_note::MidiNote;
use crate::num::ApproxEq;
use crate::sysex_event::SysExEvent;
use anyhow::{bail, Result};
use clap::Parser;
use include_dir::{include_dir, Dir};
use midly::Smf;
use std::ffi::OsStr;
use std::fs::read_dir;
use std::path::Path;

static RESOURCE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/resources");

#[allow(unused)]
pub(crate) fn show_all_midi_notes() {
    for midi_note in MidiNote::ALL {
        println!(
            "{note_number}: {name} ({frequency:.1} Hz)",
            note_number = midi_note.note_number(),
            name = midi_note.name(),
            frequency = midi_note.frequency()
        );
    }
}

#[allow(unused)]
pub(crate) fn nearest_below_or_equal() {
    let (midi_note, rem) = MidiNote::nearest_below_or_equal(880f64);
    println!("{name} + {rem} Hz", name = midi_note.name());
}

#[allow(unused)]
pub(crate) fn decode_sysex_events() -> Result<()> {
    for f in RESOURCE_DIR.files() {
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
pub(crate) fn cli() -> Result<()> {
    fn dump(path: &Path) -> Result<()> {
        match path.extension().and_then(OsStr::to_str) {
            Some("scl") => dump_scala_file(path),
            Some("syx") => dump_sysex_file(path),
            _ => Ok(()),
        }
    }

    let args = Args::parse();
    if args.start_path.is_file() {
        dump(&args.start_path)?;
    } else if args.start_path.is_dir() {
        for e in read_dir(&args.start_path)? {
            let e = e?;
            let path = args.start_path.join(e.file_name());
            dump(&path)?;
        }
    } else {
        bail!(
            "Cannot determine what {start_path} is supposed to be",
            start_path = args.start_path.display()
        )
    }
    Ok(())
}

#[allow(unused)]
pub(crate) fn generate_message() {
    // https://forums.steinberg.net/t/microtonal-midi-messages-vst-3/831268/9
    fn show_message(device_id: u8, program_number: u8, kk: u8, target_midi: f64) {
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

        let hex_dump = data
            .iter()
            .map(|x| format!("{x:02X}"))
            .collect::<Vec<_>>()
            .join(" ");
        println!("{hex_dump}");
    }

    show_message(0, 0, 0x30, 69.33f64); // just sharp of A4
}

#[allow(unused)]
pub(crate) fn misc() {
    println!("{}", BASE_FREQUENCY);
    println!("{}", BASE_MIDI_NOTE);
    println!("{:?}", 0.1f64.approx_eq(0.2f64));
}