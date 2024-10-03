use crate::args::Args;
use crate::decode_bulk_dump::decode_bulk_dump;
use crate::list_ports::list_ports;
use crate::monitor_port::monitor_port;
use crate::send_tuning::send_tuning;
use anyhow::Result;
use clap::Parser;

pub(crate) fn run() -> Result<()> {
    use crate::args::Command::*;

    match Args::parse().command {
        DecodeBulkDump { syx_path } => decode_bulk_dump(&syx_path),
        ListPorts => list_ports(),
        MonitorPort {
            midi_input_port_name,
        } => monitor_port(&midi_input_port_name),
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
