use crate::types::{Cents, Frequency};

pub(crate) const BASE_FREQUENCY: Frequency = 440f64;
pub(crate) const UNISON_CENTS: Cents = 0f64;
pub(crate) const OCTAVE_CENTS: Cents = 1200f64;
pub(crate) const DEFAULT_CENTS_EPSILON: Cents = 0.001f64;
