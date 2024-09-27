use anyhow::{bail, Error};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::BitXorAssign;
use std::result::Result as StdResult;

macro_rules! u7_lossy {
    ($value: expr) => {
        crate::u7::u7::new_lossy($value)
    };
}
pub(crate) use u7_lossy;

macro_rules! get {
    ($obj: expr) => {{
        $obj.0
    }};
}

macro_rules! set {
    ($obj: expr, $value: expr) => {{
        $obj.0 = $value
    }};
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct u7(u8);

impl u7 {
    pub(crate) const ZERO: Self = u7_lossy!(0x00);
    pub(crate) const MAX: Self = u7_lossy!(0x7f);

    pub(crate) fn to_utf8_lossy(values: &[Self]) -> String {
        let bytes = values.iter().map(|x| get!(x)).collect::<Vec<_>>();
        String::from_utf8_lossy(&bytes).into_owned()
    }

    pub(crate) const fn new_lossy(value: u8) -> Self {
        Self(value & 0x7f)
    }

    #[must_use]
    pub(crate) const fn as_u8(&self) -> u8 {
        get!(self)
    }
}

impl Display for u7 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{value:02X}", value = get!(self))
    }
}

impl TryFrom<u8> for u7 {
    type Error = Error;

    fn try_from(value: u8) -> StdResult<Self, Self::Error> {
        if value > 127 {
            bail!("Value {value} out of range for U7")
        }

        Ok(Self(value))
    }
}

impl TryFrom<u16> for u7 {
    type Error = Error;

    fn try_from(value: u16) -> StdResult<Self, Self::Error> {
        if value > 127 {
            bail!("Value {value} out of range for U7")
        }

        Ok(Self(value as u8))
    }
}

impl TryFrom<u32> for u7 {
    type Error = Error;

    fn try_from(value: u32) -> StdResult<Self, Self::Error> {
        if value > 127 {
            bail!("Value {value} out of range for U7")
        }

        Ok(Self(value as u8))
    }
}

impl TryFrom<u64> for u7 {
    type Error = Error;

    fn try_from(value: u64) -> StdResult<Self, Self::Error> {
        if value > 127 {
            bail!("Value {value} out of range for U7")
        }

        Ok(Self(value as u8))
    }
}

impl TryFrom<usize> for u7 {
    type Error = Error;

    fn try_from(value: usize) -> StdResult<Self, Self::Error> {
        if value > 127 {
            bail!("Value {value} out of range for U7")
        }

        Ok(Self(value as u8))
    }
}

impl BitXorAssign for u7 {
    fn bitxor_assign(&mut self, rhs: Self) {
        set!(self, get!(self) ^ get!(rhs))
    }
}
