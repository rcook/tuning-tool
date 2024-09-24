#![allow(unused)]

mod args;
mod cli;
mod dump_scala_file;
mod dump_sysex_file;
mod frequency;
mod fs;
mod midi_note;
mod midi_notes;
mod midi_tuning_bulk_dump_reply;
mod note;
mod notes;
mod run;
mod scala;
mod scratch;
mod tuning;

fn main() -> anyhow::Result<()> {
    crate::run::run()
}
