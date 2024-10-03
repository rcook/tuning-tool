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
            .map(|p| midi_input.port_name(&p).map_err(|e| anyhow!(e)))
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
            .map(|p| midi_output.port_name(&p).map_err(|e| anyhow!(e)))
            .collect::<Result<Vec<_>>>()?;
        names.sort();
        println!("MIDI outputs:");
        for name in names {
            println!("  {}", name);
        }
    }
    Ok(())
}
