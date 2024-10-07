use crate::frequencies::calculate_frequencies;
use crate::kbm_file::KbmFile;
use crate::scl_file::SclFile;
use anyhow::Result;

pub(crate) fn experimental() -> Result<()> {
    let scl_file = SclFile::read("resources/31edo2.scl")?;
    let kbm_file = KbmFile::read("resources/31edo2-subset.kbm")?;

    let scale = scl_file.scale();
    let keyboard_mapping = kbm_file.keyboard_mapping();

    let frequencies = calculate_frequencies(scale, keyboard_mapping)?;
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
