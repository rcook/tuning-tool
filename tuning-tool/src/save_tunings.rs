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

use crate::devices::{get_midi_output_port, make_midi_output};
use crate::midi_output_ex::MidiOutputEx;
use crate::types::MidiValue;
use anyhow::{anyhow, Result};
use midly::live::{LiveEvent, SystemCommon};
use midly::num::u7;

const NOVATION_ID: [MidiValue; 3] = [
    MidiValue::constant::<0x00>(),
    MidiValue::constant::<0x20>(),
    MidiValue::constant::<0x29>(),
];

const SAVE_TUNINGS_MESSAGE: [MidiValue; 4] = [
    MidiValue::constant::<0x00>(),
    MidiValue::constant::<0x33>(),
    MidiValue::constant::<0x00>(),
    MidiValue::constant::<0x48>(),
];

pub(crate) fn save_tunings(output_port: &str) -> Result<()> {
    let values = NOVATION_ID
        .iter()
        .chain(SAVE_TUNINGS_MESSAGE.iter())
        .copied()
        .collect::<Vec<_>>();
    let u7_slice = u7::slice_try_from_int(MidiValue::to_u8_slice(&values))
        .ok_or_else(|| anyhow!("Failed to convert slice"))?;
    let event = LiveEvent::Common(SystemCommon::SysEx(u7_slice));
    let mut message = Vec::new();
    event.write_std(&mut message)?;

    let midi_output = make_midi_output()?;
    let midi_output_port = get_midi_output_port(&midi_output, output_port)?;
    let mut conn = midi_output.connect_ex(&midi_output_port, "tuning-tool")?;
    conn.send(&message)?;
    Ok(())
}
