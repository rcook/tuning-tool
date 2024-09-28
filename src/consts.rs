use crate::midi_note::MidiNote;
use crate::types::{Cents, Frequency};
use crate::u7::{u7, u7_lossy};

pub(crate) const BASE_FREQUENCY: Frequency = 440f64;
pub(crate) const UNISON_CENTS: Cents = 0f64;
pub(crate) const OCTAVE_CENTS: Cents = 1200f64;
pub(crate) const DEFAULT_CENTS_EPSILON: Cents = 0.001f64;

pub(crate) const BASE_MIDI_NOTE: MidiNote = MidiNote::ALL[69];
pub(crate) const SYSEX: u8 = 0xf0;
pub(crate) const UNIVERSAL_NON_REAL_TIME: u7 = u7_lossy!(0x7e);
pub(crate) const MIDI_TUNING: u7 = u7_lossy!(0x08);
pub(crate) const BULK_DUMP_REPLY: u7 = u7_lossy!(0x01);
pub(crate) const BULK_DUMP_REPLY_CHECKSUM_COUNT: usize = 405;
pub(crate) const BULK_DUMP_REPLY_MESSAGE_SIZE: usize = BULK_DUMP_REPLY_CHECKSUM_COUNT + 3;
pub(crate) const EOX: u8 = 0xf7;
