use crate::frequency::Frequency;
use crate::midi_note::MidiNote;

pub(crate) const BASE_FREQUENCY: Frequency = 440f64;

pub(crate) const BASE_MIDI_NOTE: MidiNote = MidiNote::ALL[69];

pub(crate) const UNISON_CENTS: f64 = 0f64;

pub(crate) const OCTAVE_CENTS: f64 = 1200f64;

pub(crate) const DEFAULT_CENTS_EPSILON: f64 = 0.001f64;

// MIDI messages
pub(crate) const SYSEX: u8 = 0xf0;
pub(crate) const UNIVERSAL_NON_REAL_TIME: u8 = 0x7e;
pub(crate) const MIDI_TUNING: u8 = 0x08;
pub(crate) const BULK_DUMP_REPLY: u8 = 0x01;
pub(crate) const EOX: u8 = 0xf7;
