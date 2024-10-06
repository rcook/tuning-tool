use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub(crate) enum Key {
    Degree(usize),
    Unmapped,
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Degree(degree) => write!(f, "{degree}"),
            Self::Unmapped => write!(f, "(unmapped)"),
        }
    }
}
