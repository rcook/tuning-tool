use num::ToPrimitive;

pub(crate) fn foo() {
    show_message(0, 0, 0x30, 69.33f64); // just sharp of A4
}

// https://forums.steinberg.net/t/microtonal-midi-messages-vst-3/831268/9
fn show_message(device_id: u8, program_number: u8, kk: u8, target_midi: f64) {
    let xx = target_midi as i32; // i.e. 69
    let semitones = target_midi - xx as f64; // i.e. 0.33
    let semitones_14bit = (semitones * (0x4000 as f64)) as u16; // i.e. 5406
    let yy = semitones_14bit / 0x80; // i.e. 42
    let zz = semitones_14bit - 0x80 * yy; // i.e. 30

    // Single Note Tuning Change Real-Time message
    let mut data = Vec::new();

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
    data.push(0x01);

    // MIDI key number
    data.push(kk);

    // Frequency data
    data.push(xx as u8);
    data.push(yy as u8);
    data.push(zz as u8);

    // End
    data.push(0xf7);

    let hex_dump = data
        .iter()
        .map(|x| format!("{x:02X}"))
        .collect::<Vec<_>>()
        .join(" ");
    println!("{hex_dump}");
}
