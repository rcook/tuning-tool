fn main() -> anyhow::Result<()> {
    use std::env::var;
    use std::fs::File;
    use std::io::Write;
    use std::ops::Rem;
    use std::path::Path;

    const NOTES: &str = "C D EF G A B";
    const BASE_FREQUENCY: f64 = 440f64;
    const BASE_NOTE_NUMBER: i8 = 69;
    const MIDI_NOTE_COUNT: usize = 128;
    const MIDI_NOTE_TYPE_NAME: &str = "crate::midi_note::MidiNote";

    fn make_note_name(note_line: &[char], note_number: i8) -> String {
        let octave = note_number / 12 - 1;
        let index = note_number.rem(12) as usize;
        let c = note_line[index];
        if c == ' ' {
            format!("{c}#{octave}", c = note_line[index - 1])
        } else {
            format!("{c}{octave}")
        }
    }

    let note_line = NOTES.chars().collect::<Vec<_>>();

    let mut f = File::create(Path::new(&var("OUT_DIR")?).join("midi_note_generated.rs"))?;

    writeln!(
        f,
        "const ALL_MIDI_NOTES: [{MIDI_NOTE_TYPE_NAME}; {MIDI_NOTE_COUNT}] = ["
    )?;

    for i in 0..MIDI_NOTE_COUNT {
        let note_number = i as i8;
        let note_name = make_note_name(&note_line, note_number);
        let frequency =
            BASE_FREQUENCY * 2f64.powf(((note_number - BASE_NOTE_NUMBER) as f64) / 12f64);
        writeln!(
            f,
            "  {MIDI_NOTE_TYPE_NAME}::new(crate::note_number::NoteNumber::new_lossy({note_number}), \"{note_name}\", {frequency}f64),"
        )?;
    }

    writeln!(f, "];")?;

    Ok(())
}
