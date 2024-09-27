use anyhow::{bail, Error};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::BitXorAssign;
use std::result::Result as StdResult;

macro_rules! u7 {
    ($value: expr) => {
        crate::u7::U7::__unchecked_new__($value)
    };
}
pub(crate) use u7;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct U7(u8);

impl U7 {
    pub(crate) const ZERO: Self = u7!(0x00);
    pub(crate) const MAX: Self = u7!(0x7f);

    pub(crate) fn to_utf8_lossy(values: &[Self]) -> String {
        let bytes = values.iter().map(|x| x.0).collect::<Vec<_>>();
        String::from_utf8_lossy(&bytes).into_owned()
    }

    pub(crate) const fn __unchecked_new__(value: u8) -> Self {
        Self(value)
    }

    #[must_use]
    pub(crate) const fn as_u8(&self) -> u8 {
        self.0
    }
}

impl Display for U7 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{value:02X}", value = self.0)
    }
}

impl TryFrom<u8> for U7 {
    type Error = Error;

    fn try_from(value: u8) -> StdResult<Self, Self::Error> {
        if value > 127 {
            bail!("Value {value} out of range for U7")
        }

        Ok(Self(value))
    }
}

impl TryFrom<u16> for U7 {
    type Error = Error;

    fn try_from(value: u16) -> StdResult<Self, Self::Error> {
        if value > 127 {
            bail!("Value {value} out of range for U7")
        }

        Ok(Self(value as u8))
    }
}

impl TryFrom<u32> for U7 {
    type Error = Error;

    fn try_from(value: u32) -> StdResult<Self, Self::Error> {
        if value > 127 {
            bail!("Value {value} out of range for U7")
        }

        Ok(Self(value as u8))
    }
}

impl TryFrom<u64> for U7 {
    type Error = Error;

    fn try_from(value: u64) -> StdResult<Self, Self::Error> {
        if value > 127 {
            bail!("Value {value} out of range for U7")
        }

        Ok(Self(value as u8))
    }
}

impl TryFrom<usize> for U7 {
    type Error = Error;

    fn try_from(value: usize) -> StdResult<Self, Self::Error> {
        if value > 127 {
            bail!("Value {value} out of range for U7")
        }

        Ok(Self(value as u8))
    }
}

impl BitXorAssign for U7 {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}
