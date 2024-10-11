// Copyright (c) 2024 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

use crate::decode_bulk_dump::decode_bulk_dump;
use crate::dump_tuning_table::dump_tuning_table;
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
        DumpTuningTable {
            scl_path,
            kbm_path,
            output_path,
            format,
        } => dump_tuning_table(&scl_path, &kbm_path, &output_path, format),
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
