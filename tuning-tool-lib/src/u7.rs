use midly::num::u7;

pub trait U7: Sized {
    type Iter: Iterator<Item = Self>;
    const ZERO: Self;
    const ONE: Self;
    const MIN: Self;
    const MAX: Self;
    fn all() -> Self::Iter;
    fn to_u7(self) -> u7;
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

impl U7 for u7 {
    type Iter = MidlyU7Iterator;

    const ZERO: Self = u7::from_int_lossy(0);
    const ONE: Self = u7::from_int_lossy(1);
    const MIN: Self = u7::from_int_lossy(0);
    const MAX: Self = u7::from_int_lossy(128);

    fn all() -> Self::Iter {
        todo!()
    }

    fn to_u7(self) -> u7 {
        self
    }

    fn to_u8(self) -> u8 {
        self.as_int()
    }

    fn widening_succ(self) -> u8 {
        todo!()
    }

    fn widening_pred(self) -> i8 {
        todo!()
    }

    fn checked_succ(self) -> Option<Self> {
        todo!()
    }

    fn checked_pred(self) -> Option<Self> {
        todo!()
    }

    fn widening_add(self, _rhs: Self) -> u8 {
        todo!()
    }

    fn widening_sub(self, _rhs: Self) -> i8 {
        todo!()
    }

    fn checked_add(self, _rhs: Self) -> Option<Self> {
        todo!()
    }

    fn checked_sub(self, _rhs: Self) -> Option<Self> {
        todo!()
    }

    fn up_to(self, _end: Self) -> Option<Self::Iter> {
        todo!()
    }
}

pub struct MidlyU7Iterator;

impl Iterator for MidlyU7Iterator {
    type Item = u7;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
