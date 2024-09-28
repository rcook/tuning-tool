mod approx_eq;
mod args;
mod cli;
mod consts;
mod conversion;
mod dump_scala_file;
mod dump_sysex_file;
mod examples;
mod fs;
mod midi;
mod resources;
mod run;
mod scala;
mod sysex_event;
mod types;
mod u7;

fn main() -> anyhow::Result<()> {
    crate::run::run()
}
