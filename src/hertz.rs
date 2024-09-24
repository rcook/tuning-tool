use std::fmt::{Display, Formatter, Result as FmtResult};

pub(crate) struct Hertz(f64);

impl Hertz {
    #[must_use]
    pub(crate) const fn new(value: f64) -> Self {
        Self(value)
    }

    #[must_use]
    pub(crate) const fn to_f64(&self) -> f64 {
        self.0
    }
}

impl Display for Hertz {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{value} Hz", value = self.0)
    }
}
