use crate::cli::parse_absolute_path;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub(crate) struct Args {
    #[arg(help = "Start directory", value_parser = parse_absolute_path)]
    pub(crate) start_dir: PathBuf,
}
