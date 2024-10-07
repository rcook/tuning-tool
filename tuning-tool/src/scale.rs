use crate::interval::Interval;
use crate::types::Ratio;
use anyhow::{bail, Result};

#[derive(Debug)]
pub(crate) struct Scale {
    unison: Interval,
    intervals: Vec<Interval>,
}

impl Scale {
    pub(crate) fn new(intervals: Vec<Interval>) -> Result<Self> {
        if intervals.is_empty() {
            bail!("Need at least one interval");
        }
        Ok(Self {
            unison: Interval::unison(),
            intervals,
        })
    }

    pub(crate) fn unison(&self) -> &Interval {
        &self.unison
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
