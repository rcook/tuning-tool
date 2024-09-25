use anyhow::Result;

pub(crate) fn run() -> Result<()> {
    use crate::midi_note::MidiNote;

    for midi_note in MidiNote::ALL {
        println!(
            "{note_number}: {name}",
            note_number = midi_note.note_number(),
            name = midi_note.name()
        );
    }

    //println!("{}", MidiNote::nearest_below_or_equal(880f64).name());

    /*
    let bytes = read("resources/24edo.mid")?;
    let smf = Smf::parse(&bytes)?;
    for event in SysExEvent::find_all(&smf) {
        event.decode();
    }
    */

    /*
    let bytes = read("resources/24edo.mid")?;
    let smf = Smf::parse(&bytes)?;
    for event in SysExEvent::find_all(&smf) {
        event.dump();
    }
    */

    Ok(())
}
