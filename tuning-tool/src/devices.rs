use anyhow::{bail, Result};
use midir::{MidiOutput, MidiOutputPort};

const MIDI_OUTPUT_NAME: &str = "tuning-tool-midi-output";

pub(crate) fn make_midi_output() -> Result<MidiOutput> {
    Ok(MidiOutput::new(MIDI_OUTPUT_NAME)?)
}

pub(crate) fn get_midi_output_port(midi_output: &MidiOutput, name: &str) -> Result<MidiOutputPort> {
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
