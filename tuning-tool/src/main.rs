// Copyright (c) 2024 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

#![allow(clippy::wrong_self_convention)]

mod approx_eq;
mod bulk_dump_reply;
mod checksum_calculator;
mod cli;
mod consts;
mod decode_bulk_dump;
mod devices;
mod dump_tuning_table;
mod experimental;
mod frequency;
mod fs;
mod hex_dump;
mod interval;
mod kbm_file;
mod key_frequency_mapping;
mod key_mapping;
mod key_mappings;
mod keyboard_mapping;
mod list_ports;
mod midi_input_ex;
mod midi_message_builder;
mod midi_note;
mod midi_output_ex;
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
mod save_tunings;
mod scale;
mod scl_file;
mod semitones;
mod send_tuning;
mod symbolic;
mod sympy;
mod tuning_tool_args;
mod types;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    crate::run::run()
}
