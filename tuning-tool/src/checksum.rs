use tuning_tool_derive::U7;

#[derive(Clone, Copy, Debug, PartialEq, U7)]
pub(crate) struct Checksum(u8);
