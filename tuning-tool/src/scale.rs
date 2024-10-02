use crate::approx_eq::ApproxEq;
use crate::interval::Interval;
use crate::types::EquaveRatio;

#[derive(Debug)]
pub(crate) struct Scale {
    intervals: Vec<Interval>,
}

impl Scale {
    pub(crate) fn new(intervals: Vec<Interval>) -> Self {
        Self { intervals }
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
            .as_ratio()
            .approx_eq_with_epsilon(2f64, 0.0000001f64)
        {
            return false;
        }

        true
    }
}
