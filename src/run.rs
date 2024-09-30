use crate::args::{Args, Command};
use crate::consts::U7_ZERO;
use crate::send_tuning::send_tuning;
use anyhow::Result;
use clap::Parser;
use midly::num::u7;

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
            midi_port_name,
            scl_path,
            kbm_path,
            device_id,
            preset,
        } => send_tuning(&midi_port_name, &scl_path, &kbm_path, device_id, preset),
    }
}
