use crate::devices::{get_midi_output_port, make_midi_output};
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
    let mut conn = midi_output.connect(&midi_output_port, "tuning-tool")?;
    conn.send(&message)?;
    Ok(())
}
