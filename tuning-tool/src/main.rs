#![allow(clippy::wrong_self_convention)]

mod approx_eq;
mod args;
mod bulk_dump_reply;
mod checksum_calculator;
mod cli;
mod consts;
mod decode_bulk_dump;
mod devices;
mod frequencies;
mod frequency;
mod fs;
mod hex_dump;
mod interval;
mod kbm_file;
mod key;
mod keyboard_mapping;
mod list_ports;
mod midi_message_builder;
mod midi_note;
mod monitor_port;
mod mts_entry;
mod note_change;
mod note_change_entry;
mod note_number;
mod num;
mod preset_name;
mod read;
mod resources;
mod run;
mod scale;
mod scl_file;
mod semitones;
mod send_tuning;
mod test_util;
mod types;

fn main() -> anyhow::Result<()> {
    crate::run::run()
}
