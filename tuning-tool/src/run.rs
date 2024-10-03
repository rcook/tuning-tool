use crate::args::Args;
use crate::list_ports::list_ports;
use crate::monitor_port::monitor_port;
use crate::send_tuning::send_tuning;
use anyhow::Result;
use clap::Parser;

pub(crate) fn run() -> Result<()> {
    use crate::args::Command::*;

    match Args::parse().command {
        ListPorts => list_ports(),
        MonitorPort {
            midi_input_port_name,
        } => monitor_port(&midi_input_port_name),
        Other => {
            //crate::examples::show_all_midi_notes();
            //crate::examples::nearest_below_or_equal();
            //crate::examples::decode_sysex_events()?;
            //crate::examples::cli()?;
            //crate::examples::generate_message();
            //crate::examples::misc();
            crate::examples::play_note()
        }
        SendTuning {
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
