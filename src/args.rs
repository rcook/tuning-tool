use clap::Parser;
use path_absolutize::Absolutize;
use std::path::PathBuf;
use std::result::Result as StdResult;

#[derive(Parser)]
pub(crate) struct Args {
    #[arg(help = "Start directory", value_parser = parse_absolute_path)]
    pub(crate) start_dir: PathBuf,
}

fn parse_absolute_path(s: &str) -> StdResult<PathBuf, String> {
    PathBuf::from(s)
        .absolutize()
        .map_err(|_| String::from("Invalid path"))
        .map(|p| p.to_path_buf())
}
