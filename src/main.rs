mod args;
mod cli;
mod consts;
mod dump_scala_file;
mod dump_sysex_file;
mod examples;
mod frequency;
mod fs;
mod midi_note;
mod midi_tuning_bulk_dump_reply;
mod note_number;
mod num;
mod run;
mod scala;
mod sysex_event;

fn main() -> anyhow::Result<()> {
    crate::run::run()
}
