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

    fn make_note_info(note_line: &[char], note_number: i8) -> (String, bool) {
        let octave = note_number / 12 - 1;
        let index = note_number.rem(12) as usize;
        let c = note_line[index];
        if c == ' ' {
            (format!("{c}#{octave}", c = note_line[index - 1]), false)
        } else {
            (format!("{c}{octave}"), true)
        }
    }

    let note_line = NOTES.chars().collect::<Vec<_>>();

    let mut f = File::create(Path::new(&var("OUT_DIR")?).join("midi_note_generated.rs"))?;

    writeln!(
        f,
        "const ALL: [{MIDI_NOTE_TYPE_NAME}; {MIDI_NOTE_COUNT}] = ["
    )?;

    for i in 0..MIDI_NOTE_COUNT {
        let note_number = i as i8;
        let (note_name, is_natural) = make_note_info(&note_line, note_number);
        let frequency =
            BASE_FREQUENCY * 2f64.powf(((note_number - BASE_NOTE_NUMBER) as f64) / 12f64);
        writeln!(
            f,
            "  {MIDI_NOTE_TYPE_NAME}::new(crate::note_number::NoteNumber::constant::<{note_number}>(), \"{note_name}\", {is_natural}, crate::frequency::Frequency({frequency}f64)),"
        )?;
    }

    writeln!(f, "];")?;

    Ok(())
}
