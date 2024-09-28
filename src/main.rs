#![allow(clippy::wrong_self_convention)]
#![allow(unused)]

mod approx_eq;
mod args;
mod cent_offset;
mod cents;
mod cli;
mod consts;
mod dump_sysex_file;
mod examples;
mod frequency;
mod fs;
mod interval;
mod midi;
mod mts_bytes;
mod note_number;
mod num;
mod ratio;
mod resources;
mod run;
mod scala_file;
mod scale;
mod semitones;
mod sysex_event;
mod tuning;
mod types;
mod u7;

fn main() -> anyhow::Result<()> {
    crate::run::run()
}
