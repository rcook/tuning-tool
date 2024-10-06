use crate::cli::parse_absolute_path;
use crate::types::{ChunkSize, DeviceId, Preset};
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Parser)]
#[command(about = "Richard's MIDI Tuning Tool")]
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
