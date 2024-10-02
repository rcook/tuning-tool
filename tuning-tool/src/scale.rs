use crate::interval::Interval;
use crate::ratio::Ratio;
use anyhow::{bail, Result};

#[derive(Debug)]
pub(crate) struct Scale {
    intervals: Vec<Interval>,
}

impl Scale {
    pub(crate) fn new(intervals: Vec<Interval>) -> Result<Self> {
        if intervals.is_empty() {
            bail!("Need at least one interval");
        }
        Ok(Self { intervals })
    }

    pub(crate) fn intervals(&self) -> &Vec<Interval> {
        &self.intervals
    }

    pub(crate) fn equave_ratio(&self) -> Ratio {
        let last_interval = self
            .intervals
            .last()
            .expect("Must have at least one interval");
        last_interval.as_ratio()
    }
}
