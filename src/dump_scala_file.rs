use crate::scala::fs::read_scala_file;
use anyhow::Result;
use std::path::Path;

pub(crate) fn dump_scala_file(scl_path: &Path) -> Result<()> {
    println!("Testing {}", scl_path.display());
    let tuning = read_scala_file(scl_path)?;

    if let Some(file_name) = tuning.file_name() {
        println!("File name: {file_name}");
    }

    println!(
        "Description: {description}",
        description = tuning.description()
    );

    println!("Steps: {step_count}", step_count = tuning.step_count());
    println!("Notes: {note_count}", note_count = tuning.note_count());

    for (i, note) in tuning.notes().enumerate() {
        println!("(note {i}): {cents}", cents = note.cents());
    }

    Ok(())
}
