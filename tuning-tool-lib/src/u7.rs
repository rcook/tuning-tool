use crate::TryFromU8Error;

pub trait U7: Copy + Sized + TryFrom<u8, Error = TryFromU8Error> {
    type Iter: Iterator<Item = Self>;
    const ZERO: Self;
    const ONE: Self;
    const MIN: Self;
    const MAX: Self;
    fn all() -> Self::Iter;
    fn to_u8(self) -> u8;
    fn widening_succ(self) -> u8;
    fn widening_pred(self) -> i8;
    fn checked_succ(self) -> Option<Self>;
    fn checked_pred(self) -> Option<Self>;
    fn widening_add(self, rhs: Self) -> u8;
    fn widening_sub(self, rhs: Self) -> i8;
    fn checked_add(self, rhs: Self) -> Option<Self>;
    fn checked_sub(self, rhs: Self) -> Option<Self>;
    fn up_to(self, end: Self) -> Option<Self::Iter>;
}
