mod args;
mod cli;
mod consts;
mod dump_scala_file;
mod dump_sysex_file;
mod examples;
mod frequency;
mod fs;
mod midi_bulk_tuning_dump_reply;
mod midi_note;
mod midi_note_number;
mod num;
mod resources;
mod run;
mod scala;
mod sysex_event;

fn main() -> anyhow::Result<()> {
    crate::run::run()
}
