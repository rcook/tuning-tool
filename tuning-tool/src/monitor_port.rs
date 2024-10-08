use crate::devices::{get_midi_input_port, make_midi_input};
use crate::hex_dump::to_hex_dump;
use crate::midi_input_ex::MidiInputEx;
use anyhow::{Error, Result};
use std::sync::mpsc::{channel, Sender};

pub(crate) fn monitor_port(input_port: &str) -> Result<()> {
    fn callback_wrapper(timestamp: u64, bytes: &[u8], tx: &mut Sender<Error>) {
        if let Err(e) = callback(timestamp, bytes) {
            tx.send(e).expect("Send failed");
        }
    }

    fn callback(timestamp: u64, bytes: &[u8]) -> Result<()> {
        println!(
            "{timestamp:08x} {hex_dump}",
            hex_dump = to_hex_dump(bytes, None)?
        );
        Ok(())
    }

    let midi_input = make_midi_input()?;
    let midi_input_port = get_midi_input_port(&midi_input, input_port)?;
    let (tx, rx) = channel();
    let _conn = midi_input.connect_ex(&midi_input_port, "tuning-tool", callback_wrapper, tx)?;
    let e = rx.recv()?;
    println!("Failed with error {e:?}");
    Ok(())
}
