use crate::kbm_file::KbmFile;
use crate::key_frequency_mapping::KeyFrequencyMapping;
use crate::scl_file::SclFile;
use anyhow::Result;

pub(crate) fn experimental() -> Result<()> {
    let scl_file = SclFile::read("resources/31edo2.scl")?;
    let kbm_file = KbmFile::read("resources/31edo2-subset.kbm")?;
    let scale = scl_file.scale();
    let keyboard_mapping = kbm_file.keyboard_mapping();
    let mappings = KeyFrequencyMapping::compute(scale, keyboard_mapping)?;

    println!("const EXPECTED_FREQUENCIES: [f64; 128] = [");
    for mapping in mappings {
        println!("  {f}f64,", f = mapping.frequency);
    }
    println!("];");

    Ok(())
}
