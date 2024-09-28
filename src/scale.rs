use crate::approx_eq::ApproxEq;
use crate::consts::{DEFAULT_CENTS_EPSILON, OCTAVE_CENTS, UNISON_CENTS};
use crate::interval::Interval;

#[derive(Debug)]
pub(crate) struct Scale {
    intervals: Vec<Interval>,
}

impl Scale {
    pub(crate) fn new(intervals: Vec<Interval>) -> Self {
        Self { intervals }
    }

    pub(crate) fn step_count(&self) -> usize {
        self.interval_count() - 1
    }

    pub(crate) fn interval_count(&self) -> usize {
        self.intervals.len()
    }

    pub(crate) fn intervals(&self) -> &Vec<Interval> {
        &self.intervals
    }

    pub(crate) fn is_octave_repeating(&self) -> bool {
        let Some(first_note) = self.intervals.first() else {
            return false;
        };

        if !first_note
            .cents()
            .approx_eq_with_epsilon(UNISON_CENTS, DEFAULT_CENTS_EPSILON)
        {
            return false;
        }

        let Some(last_note) = self.intervals.last() else {
            return false;
        };

        if !last_note
            .cents()
            .approx_eq_with_epsilon(OCTAVE_CENTS, DEFAULT_CENTS_EPSILON)
        {
            return false;
        }

        true
    }
}
