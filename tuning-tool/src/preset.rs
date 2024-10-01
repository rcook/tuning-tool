use std::fmt::{Display, Formatter, Result as FmtResult};
use tuning_tool_derive::U7;

#[derive(Clone, Copy, Debug, PartialEq, U7)]
pub(crate) struct Preset(u8);

impl Display for Preset {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}
