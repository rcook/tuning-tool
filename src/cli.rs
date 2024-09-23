use path_absolutize::Absolutize;
use std::path::PathBuf;
use std::result::Result as StdResult;

pub(crate) fn parse_absolute_path(s: &str) -> StdResult<PathBuf, String> {
    PathBuf::from(s)
        .absolutize()
        .map_err(|_| String::from("Invalid path"))
        .map(|p| p.to_path_buf())
}
