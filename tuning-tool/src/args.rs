use crate::cli::parse_absolute_path;
use crate::types::{ChunkSize, DeviceId, Preset};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Parser)]
#[command(about = "Richard's MIDI Tuning Tool")]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    #[command(name = "list-devices", about = "List MIDI devices")]
    ListDevices,

    #[command(name = "other", about = "Other")]
    Other {
        #[arg(help = "Start directory or file", value_parser = parse_absolute_path)]
        start_path: PathBuf,
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

        #[arg(long = "output", short = 'o', help = "MIDI output port name")]
        midi_output_port_name: Option<String>,

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
