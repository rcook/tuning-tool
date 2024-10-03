use anyhow::{bail, Result};
use midir::{MidiInput, MidiInputPort, MidiOutput, MidiOutputPort};

const MIDI_INPUT_NAME: &str = "tuning-tool-midi-input";
const MIDI_OUTPUT_NAME: &str = "tuning-tool-midi-output";

pub(crate) fn make_midi_input() -> Result<MidiInput> {
    Ok(MidiInput::new(MIDI_INPUT_NAME)?)
}

pub(crate) fn make_midi_output() -> Result<MidiOutput> {
    Ok(MidiOutput::new(MIDI_OUTPUT_NAME)?)
}

pub(crate) fn get_midi_input_port(midi_input: &MidiInput, name: &str) -> Result<MidiInputPort> {
    let mut names = Vec::new();
    for p in midi_input.ports() {
        let temp = midi_input.port_name(&p)?;
        if midi_input.port_name(&p)? == name {
            return Ok(p);
        }
        names.push(temp);
    }

    let s = names.join(", ");
    bail!("No MIDI input port with name {name} found: choose from {s}");
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
