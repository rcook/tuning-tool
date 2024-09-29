use midly::num::u7;
use std::str;

pub(crate) trait StringExtras<'a> {
    type Output;

    fn from_u7_slice(slice: &'a [u7]) -> Self::Output;
}

impl<'a> StringExtras<'a> for str {
    type Output = &'a str;

    fn from_u7_slice(slice: &'a [u7]) -> &'a str {
        let bytes = u7::slice_as_int(slice);
        str::from_utf8(bytes).expect("Array of u7 values must be valid UTF-8")
    }
}

impl<'a> StringExtras<'a> for String {
    type Output = String;

    fn from_u7_slice(slice: &[u7]) -> String {
        String::from(str::from_u7_slice(slice))
    }
}
