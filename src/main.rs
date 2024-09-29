#![allow(clippy::wrong_self_convention)]

mod approx_eq;
mod args;
mod bulk_dump_reply;
mod cent_offset;
mod cents;
mod checksum_calculator;
mod cli;
mod consts;
mod dump_sysex_file;
mod equave_ratio;
mod examples;
mod frequencies;
mod frequency;
mod fs;
mod hex_dump;
mod interval;
mod midi_note;
mod mts_entry;
mod note_change;
mod note_change_entry;
mod note_number;
mod num;
mod preset_name;
mod ratio;
mod resources;
mod run;
mod scala_file;
mod scale;
mod semitones;
mod send_tuning;
mod string_extras;
mod sysex_event;
mod test_util;

fn main() -> anyhow::Result<()> {
    crate::run::run()
}
