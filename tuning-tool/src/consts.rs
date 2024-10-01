use crate::midi_value::MidiValue;

pub(crate) const SYSEX: u8 = 0xf0;
pub(crate) const UNIVERSAL_REAL_TIME: MidiValue = MidiValue::constant::<0x7f>();
pub(crate) const UNIVERSAL_NON_REAL_TIME: MidiValue = MidiValue::constant::<0x7e>();
pub(crate) const MIDI_TUNING: MidiValue = MidiValue::constant::<8>();
pub(crate) const NOTE_CHANGE: MidiValue = MidiValue::constant::<2>();
pub(crate) const BULK_DUMP_REPLY: MidiValue = MidiValue::constant::<1>();
pub(crate) const BULK_DUMP_REPLY_CHECKSUM_COUNT: usize = 405;
pub(crate) const BULK_DUMP_REPLY_MESSAGE_SIZE: usize = BULK_DUMP_REPLY_CHECKSUM_COUNT + 1;
pub(crate) const EOX: u8 = 0xf7;
