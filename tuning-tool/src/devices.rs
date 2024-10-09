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
