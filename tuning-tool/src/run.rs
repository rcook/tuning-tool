use crate::decode_bulk_dump::decode_bulk_dump;
use crate::experimental::experimental;
use crate::list_ports::list_ports;
use crate::monitor_port::monitor_port;
use crate::save_tunings::save_tunings;
use crate::send_tuning::send_tuning;
use crate::tuning_tool_args::Command::*;
use crate::tuning_tool_args::TuningToolArgs;
use anyhow::Result;
use clap::Parser;

pub(crate) fn run() -> Result<()> {
    match TuningToolArgs::parse().command {
        DecodeBulkDump { syx_path } => decode_bulk_dump(&syx_path),
        Experimental => experimental(),
        ListPorts => list_ports(),
        MonitorPort { input_port } => monitor_port(&input_port),
        SaveTunings { output_port } => save_tunings(&output_port),
        SendTuning {
            scl_path,
            kbm_path,
            output,
            device_id,
            preset,
            chunk_size,
        } => send_tuning(&scl_path, &kbm_path, &output, device_id, preset, chunk_size),
    }
}
