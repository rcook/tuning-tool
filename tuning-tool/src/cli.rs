use midly::num::u7;
use path_absolutize::Absolutize;
use std::path::PathBuf;
use std::result::Result as StdResult;

pub(crate) fn parse_absolute_path(s: &str) -> StdResult<PathBuf, String> {
    PathBuf::from(s)
        .absolutize()
        .map_err(|_| String::from("Invalid path"))
        .map(|p| p.to_path_buf())
}

pub(crate) fn parse_u7(s: &str) -> StdResult<u7, String> {
    #[allow(clippy::unnecessary_fallible_conversions)]
    s.parse::<u8>()
        .map_err(|_| String::from("Invalid u8 value"))?
        .try_into()
        .map_err(|_| String::from("Invalid u7 value"))
}
