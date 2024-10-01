use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TryFromU8Error {
    #[error("value {0} out of range for type")]
    OutOfRange(u8),
}

#[derive(Debug, Error)]
pub enum FromStrError {
    #[error("value {0} out of range for type")]
    OutOfRange(u8),

    #[error(transparent)]
    Other(#[from] ParseIntError),
}
