use std::str;
use tuning_tool_lib::U7;

use crate::coerce::unsafe_coerce_slice_to_u8_slice;

pub(crate) trait StringExtras<'a> {
    type Output;

    fn from_u7_slice<U: U7>(slice: &'a [U]) -> Self::Output;
}

impl<'a> StringExtras<'a> for str {
    type Output = &'a str;

    fn from_u7_slice<U: U7>(slice: &'a [U]) -> &'a str {
        str::from_utf8(unsafe_coerce_slice_to_u8_slice(slice))
            .expect("Array of u7 values must be valid UTF-8")
    }
}

impl<'a> StringExtras<'a> for String {
    type Output = String;

    fn from_u7_slice<U: U7>(slice: &[U]) -> String {
        String::from(str::from_u7_slice(slice))
    }
}
