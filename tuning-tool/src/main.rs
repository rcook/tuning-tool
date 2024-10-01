#![allow(clippy::wrong_self_convention)]

mod approx_eq;
mod args;
mod bulk_dump_reply;
mod cent_offset;
mod cents;
mod checksum;
mod checksum_calculator;
mod chunk_size;
mod cli;
mod consts;
mod device_id;
mod dump_sysex_file;
mod equave_ratio;
mod examples;
mod frequencies;
mod frequency;
mod fs;
mod hex_dump;
mod interval;
mod kbm_file;
mod key;
mod keyboard_mapping;
mod midi_note;
mod mts_entry;
mod note_change;
mod note_change_entry;
mod note_number;
mod num;
mod preset;
mod preset_name;
mod ratio;
mod resources;
mod run;
mod scale;
mod scl_file;
mod semitones;
mod send_tuning;
mod string_extras;
mod sysex_event;
mod test_util;
mod u7;

fn main() -> anyhow::Result<()> {
    crate::run::run()
}
