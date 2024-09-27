use crate::midi::midi_note::MidiNote;
use crate::u7::{u7_lossy, U7};

pub(crate) const BASE_MIDI_NOTE: MidiNote = MidiNote::ALL[69];
pub(crate) const SYSEX: u8 = 0xf0;
pub(crate) const UNIVERSAL_NON_REAL_TIME: U7 = u7_lossy!(0x7e);
pub(crate) const MIDI_TUNING: U7 = u7_lossy!(0x08);
pub(crate) const BULK_DUMP_REPLY: U7 = u7_lossy!(0x01);
pub(crate) const BULK_DUMP_REPLY_CHECKSUM_COUNT: usize = 405;
pub(crate) const BULK_DUMP_REPLY_MESSAGE_SIZE: usize = BULK_DUMP_REPLY_CHECKSUM_COUNT + 3;
pub(crate) const EOX: u8 = 0xf7;
