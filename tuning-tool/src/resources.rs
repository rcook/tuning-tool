#![cfg(test)]

use include_dir::{include_dir, Dir};

pub(crate) static RESOURCE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../resources");
