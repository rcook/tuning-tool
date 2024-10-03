use crate::TryFromU8Error;

pub trait U7: Copy + Sized + TryFrom<u8, Error = TryFromU8Error> {
    const ZERO: Self;
    fn to_u8(self) -> u8;
}
