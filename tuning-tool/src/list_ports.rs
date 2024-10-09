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

use crate::devices::{make_midi_input, make_midi_output};
use anyhow::{anyhow, Result};

pub(crate) fn list_ports() -> Result<()> {
    let midi_input = make_midi_input()?;
    if midi_input.port_count() == 0 {
        println!("(You have no MIDI input ports)");
    } else {
        let mut names = midi_input
            .ports()
            .iter()
            .map(|p| midi_input.port_name(p).map_err(|e| anyhow!(e)))
            .collect::<Result<Vec<_>>>()?;
        names.sort();
        println!("MIDI inputs:");
        for name in names {
            println!("  {}", name);
        }
    }

    let midi_output = make_midi_output()?;
    if midi_output.port_count() == 0 {
        println!("(You have no MIDI output ports)");
    } else {
        let mut names = midi_output
            .ports()
            .iter()
            .map(|p| midi_output.port_name(p).map_err(|e| anyhow!(e)))
            .collect::<Result<Vec<_>>>()?;
        names.sort();
        println!("MIDI outputs:");
        for name in names {
            println!("  {}", name);
        }
    }
    Ok(())
}
