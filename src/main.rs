mod args;
mod cli;
mod consts;
mod dump_scala_file;
mod dump_sysex_file;
mod examples;
mod fs;
mod midi;
mod num;
mod resources;
mod run;
mod scala;
mod sysex_event;
mod types;

fn main() -> anyhow::Result<()> {
    crate::run::run()
}
