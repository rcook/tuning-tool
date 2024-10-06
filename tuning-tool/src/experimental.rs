use crate::frequencies::calculate_frequencies;
use crate::frequency::Frequency;
use crate::kbm_file::KbmFile;
use crate::keyboard_mapping::KeyboardMapping;
use crate::note_number::NoteNumber;
use crate::scl_file::SclFile;
use anyhow::Result;
use tuning_tool_macros::scale;

pub(crate) fn experimental() -> Result<()> {
    let scl_file = SclFile::read("resources/weird.scl")?;
    println!("{scl_file:?}");
    let kbm_file = KbmFile::read("resources/weird.kbm")?;
    println!("{kbm_file:?}");
    Ok(())
}

#[allow(unused)]
pub(crate) fn show_scale_frequencies() -> Result<()> {
    let bohlen_p = scale![
        27/25
        25/21
        9/7
        7/5
        75/49
        5/3
        9/5
        49/25
        15/7
        7/3
        63/25
        25/9
        3/1
    ];

    let keyboard_mapping = KeyboardMapping::new(
        NoteNumber::ZERO,
        NoteNumber::MAX,
        NoteNumber::A4,
        Frequency::A4,
    )?;

    let frequencies = calculate_frequencies(&bohlen_p, &keyboard_mapping);
    for (i, f) in frequencies.iter().enumerate() {
        println!("{i:>3}: {f} Hz,");
    }
    println!("const EXPECTED_FREQUENCIES: [f64; 128] = [");
    for f in frequencies {
        println!("  {f}f64,");
    }
    println!("];");

    Ok(())
}
