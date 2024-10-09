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
