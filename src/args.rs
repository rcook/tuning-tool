use crate::cli::parse_absolute_path;
use clap::{Parser, Subcommand};
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
        #[arg(help = "MIDI port name")]
        midi_port_name: String,

        #[arg(help = "Path to .scl file", value_parser = parse_absolute_path)]
        scl_path: PathBuf,
    },
}
