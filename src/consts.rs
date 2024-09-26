use crate::frequency::Frequency;
use crate::midi_note::MidiNote;

pub(crate) const BASE_FREQUENCY: Frequency = 440f64;

pub(crate) const BASE_MIDI_NOTE: MidiNote = MidiNote::ALL[69];

pub(crate) const UNISON_CENTS: f64 = 0f64;

pub(crate) const OCTAVE_CENTS: f64 = 1200f64;

pub(crate) const DEFAULT_CENTS_EPSILON: f64 = 0.001f64;
