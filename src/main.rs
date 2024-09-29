#![allow(clippy::wrong_self_convention)]
#![allow(unused)]

mod approx_eq;
mod args;
mod bulk_tuning_dump_reply;
mod cent_offset;
mod cents;
mod checksum_calculator;
mod cli;
mod consts;
mod dump_sysex_file;
mod examples;
mod frequency;
mod fs;
mod interval;
mod midi_note;
mod mts_bytes;
mod note_number;
mod num;
mod ratio;
mod resources;
mod run;
mod scala_file;
mod scale;
mod semitones;
mod string_extras;
mod sysex_event;
mod tuning;
mod types;

fn main() -> anyhow::Result<()> {
    crate::run::run()
}
