use midly::num::u7;

pub(crate) const SYSEX: u8 = 0xf0;
pub(crate) const UNIVERSAL_REAL_TIME: u7 = u7::from_int_lossy(0x7f);
pub(crate) const UNIVERSAL_NON_REAL_TIME: u7 = u7::from_int_lossy(0x7e);
pub(crate) const MIDI_TUNING: u7 = u7::from_int_lossy(0x08);
pub(crate) const NOTE_CHANGE: u7 = u7::from_int_lossy(2);
pub(crate) const BULK_DUMP_REPLY: u7 = u7::from_int_lossy(0x01);
pub(crate) const BULK_DUMP_REPLY_CHECKSUM_COUNT: usize = 405;
pub(crate) const BULK_DUMP_REPLY_MESSAGE_SIZE: usize = BULK_DUMP_REPLY_CHECKSUM_COUNT + 1;
pub(crate) const EOX: u8 = 0xf7;

pub(crate) const U7_ZERO: u7 = u7::from_int_lossy(0x00);
