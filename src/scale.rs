use crate::approx_eq::ApproxEq;
use crate::consts::{DEFAULT_CENTS_EPSILON, OCTAVE_CENTS, UNISON_CENTS};
use crate::interval::Interval;
use crate::tuning::EquaveRatio;

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

    pub(crate) fn equave_ratio(&self) -> EquaveRatio {
        assert!(self.is_octave_repeating());
        EquaveRatio(2f64) // TBD
    }

    fn is_octave_repeating(&self) -> bool {
        let Some(last_interval) = self.intervals.last() else {
            return false;
        };

        if !last_interval
            .to_f64()
            .approx_eq_with_epsilon(2f64, 0.0001f64)
        {
            return false;
        }

        true
    }
}
