use crate::args::{Args, Command};
use crate::send_tuning::send_tuning;
use anyhow::Result;
use clap::Parser;

pub(crate) fn run() -> Result<()> {
    match Args::parse().command {
        Command::Other { .. } => {
            //crate::examples::show_all_midi_notes();
            //crate::examples::nearest_below_or_equal();
            //crate::examples::decode_sysex_events()?;
            //crate::examples::cli()?;
            //crate::examples::generate_message();
            //crate::examples::misc();
            //crate::examples::enumerate_midi_outputs()?;
            //crate::examples::play_note()?;
            crate::examples::send_tuning_sysex()
        }
        Command::SendTuning {
            scl_path,
            kbm_path,
            midi_output_port_name,
            device_id,
            preset,
            chunk_size,
        } => send_tuning(
            &scl_path,
            &kbm_path,
            &midi_output_port_name,
            device_id,
            preset,
            chunk_size,
        ),
    }
}
