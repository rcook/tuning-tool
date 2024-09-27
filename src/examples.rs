use crate::args::Args;
use crate::consts::BASE_FREQUENCY;
use crate::dump_scala_file::dump_scala_file;
use crate::dump_sysex_file::dump_sysex_file;
use crate::midi::bulk_tuning_dump_reply::BulkTuningDumpReply;
use crate::midi::consts::BASE_MIDI_NOTE;
use crate::midi::midi_frequency::MidiFrequency;
use crate::midi::midi_note::MidiNote;
use crate::num::ApproxEq;
use crate::resources::RESOURCE_DIR;
use crate::scala::tuning::Tuning;
use crate::sysex_event::SysExEvent;
use anyhow::{anyhow, bail, Result};
use clap::Parser;
use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};
use midly::Smf;
use std::ffi::OsStr;
use std::fs::read_dir;
use std::iter::zip;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

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
    let (midi_note, rem) = MidiNote::nearest_below_or_equal(880f64).expect("Must succeed");
    println!("{name} + {rem} Hz", name = midi_note.name());
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
pub(crate) fn send_octave_repeating_tuning() -> Result<()> {
    let scl_dir = RESOURCE_DIR
        .get_dir("scl")
        .ok_or_else(|| anyhow!("Could not get scl directory"))?;
    let scl_file = scl_dir
        .get_file("scl/carlos_super.scl")
        .ok_or_else(|| anyhow!("Could not get scl file"))?;
    let s = scl_file
        .contents_utf8()
        .ok_or_else(|| anyhow!("Could not convert to string"))?;
    let tuning = s.parse::<Tuning>()?;

    if !tuning.is_octave_repeating() {
        bail!("Scale is not octave-repeating");
    }

    let step_count = tuning.step_count();

    let frequencies = zip(
        MidiNote::ALL.iter(),
        tuning.notes().take(step_count).cycle(),
    )
    .map(|(midi_note, scale_note)| {
        let octave = ((midi_note.note_number() as usize) / step_count).try_into()?;
        let cents = scale_note.cents().expect("Must succeed");
        MidiFrequency::from_cents(octave, cents)
    })
    .collect::<Result<Vec<_>>>()?
    .try_into()
    .expect("Must have exactly 128 elements");

    let reply = BulkTuningDumpReply::new(0, 0, "HELLO", frequencies)?;
    let bytes = reply.to_bytes()?;

    let ref_bytes = RESOURCE_DIR
        .get_file("syx/carlos_super.syx")
        .ok_or_else(|| anyhow!("Could not load tuning dump"))?
        .contents()
        .to_vec();

    assert_eq!(ref_bytes.len(), bytes.len());

    for (a, b) in zip(ref_bytes, bytes) {
        if a == b {
            println!("{a:02X}")
        } else {
            println!("{a:02X} vs {b:02X}")
        }
    }

    Ok(())
}
