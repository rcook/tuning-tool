use crate::cli::{parse_absolute_path, parse_u7};
use crate::consts::U7_ZERO;
use clap::{Parser, Subcommand};
use midly::num::u7;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(about = "Richard's MIDI Tuning Tool")]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
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
            value_parser = parse_u7,
            default_value_t = U7_ZERO
        )]
        device_id: u7,

        #[arg(
            help = "Preset",
            long = "preset",
            short = 'p',
            value_parser = parse_u7,
            default_value_t = u7::from_int_lossy(8)
        )]
        preset: u7,

        #[arg(
            help = "Chunk size",
            long = "chunk",
            short = 'c',
            value_parser = parse_u7,
            default_value_t = u7::from_int_lossy(1)
        )]
        chunk_size: u7,
    },
}
