use anyhow::Result;

pub(crate) fn run() -> Result<()> {
    //crate::examples::show_all_midi_notes();
    //crate::examples::nearest_below_or_equal();
    //crate::examples::decode_sysex_events()?;
    //crate::examples::cli()?;
    //crate::examples::generate_message();
    //crate::examples::misc();
    crate::examples::enumerate_midi_outputs()?;
    Ok(())
}
