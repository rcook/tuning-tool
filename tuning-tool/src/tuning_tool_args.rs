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

use crate::cli::parse_absolute_path;
use crate::types::{ChunkSize, DeviceId, Preset};
use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use std::str::FromStr;

const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const PACKAGE_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
const PACKAGE_HOME_PAGE: &str = env!("CARGO_PKG_HOMEPAGE");
const PACKAGE_BUILD_VERSION: Option<&str> = option_env!("RUST_TOOL_ACTION_BUILD_VERSION");

#[derive(Debug, Parser)]
#[command(
    name = PACKAGE_NAME,
    version = PACKAGE_VERSION,
    about = format!("{PACKAGE_DESCRIPTION} {PACKAGE_VERSION}"),
    after_help = format!(
        "{PACKAGE_HOME_PAGE}{end}",
        end = PACKAGE_BUILD_VERSION
            .map(|x| format!("\n\n{}", x))
            .unwrap_or_else(|| String::from("")))
)]
pub(crate) struct TuningToolArgs {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    #[command(
        name = "decode-bulk-dump",
        about = "Decode MIDI bulk tuning dump reply"
    )]
    DecodeBulkDump {
        #[arg(
            help = "Path to .syx file",
            value_parser = parse_absolute_path
        )]
        syx_path: PathBuf,
    },

    #[command(name = "dump-tuning-table", about = "Dump tuning table to text file")]
    DumpTuningTable {
        #[arg(
            help = "Path to .scl file",
            value_parser = parse_absolute_path
        )]
        scl_path: PathBuf,

        #[arg(
            help = "Path to .kbm file",
            value_parser = parse_absolute_path
        )]
        kbm_path: PathBuf,

        #[arg(
            long = "output",
            short = 'o',
            help = "Output path",
            value_parser = parse_absolute_path
        )]
        output_path: Option<PathBuf>,

        #[arg(
            long = "format",
            short = 'f',
            help = "Output format",
            default_value = "detailed"
        )]
        format: DumpTuningTableFormat,
    },

    #[command(name = "experimental", about = "Experimental stuff")]
    Experimental,

    #[command(name = "list-ports", about = "List MIDI input and output ports")]
    ListPorts,

    #[command(name = "monitor-port", about = "Monitor MIDI input port")]
    MonitorPort {
        #[arg(help = "MIDI input port name")]
        input_port: String,
    },

    #[command(
        name = "save-tunings",
        about = "Save tuning tables on Novation Bass Station II"
    )]
    SaveTunings {
        #[arg(help = "MIDI output port name")]
        output_port: String,
    },

    #[command(name = "send-tuning", about = "Send tuning SysEx to MIDI device")]
    SendTuning {
        #[arg(
            help = "Path to .scl file",
            value_parser = parse_absolute_path
        )]
        scl_path: PathBuf,

        #[arg(
            help = "Path to .kbm file",
            value_parser = parse_absolute_path
        )]
        kbm_path: PathBuf,

        #[command(flatten)]
        output: SendTuningOutput,

        #[arg(
            help = "Device ID",
            long = "device",
            short = 'd',
            value_parser = <DeviceId as FromStr>::from_str,
            default_value_t = DeviceId::ZERO
        )]
        device_id: DeviceId,

        #[arg(
            help = "Preset",
            long = "preset",
            short = 'p',
            value_parser = <Preset as FromStr>::from_str,
            default_value_t = Preset::constant::<8>()
        )]
        preset: Preset,

        #[arg(
            help = "Chunk size",
            long = "chunk",
            short = 'c',
            value_parser = <ChunkSize as FromStr>::from_str,
            default_value_t = ChunkSize::ONE
        )]
        chunk_size: ChunkSize,
    },
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
pub(crate) struct SendTuningOutput {
    #[arg(long = "output", short = 'o', help = "MIDI output port name")]
    pub(crate) output_port: Option<String>,

    #[arg(long = "file", short = 'f', help = "Path to SysEx file")]
    pub(crate) syx_path: Option<String>,
}

#[derive(Clone, Debug, ValueEnum)]
pub(crate) enum DumpTuningTableFormat {
    #[clap(name = "brief")]
    Brief,
    #[clap(name = "detailed")]
    Detailed,
}
