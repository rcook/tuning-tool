#![allow(clippy::wrong_self_convention)]

mod approx_eq;
mod args;
mod cli;
mod consts;
mod conversion;
mod dump_scala_file;
mod dump_sysex_file;
mod examples;
mod fs;
mod interval;
mod midi;
mod resources;
mod run;
mod scale;
mod sysex_event;
mod tuning;
mod types;
mod u7;

fn main() -> anyhow::Result<()> {
    crate::run::run()
}
