use std::default;

use derive_builder::Builder;

#[derive(Builder, Default)]
#[builder(default)]
pub struct HexDumpOptions {
    pub columns: Option<usize>,
}

pub(crate) fn hex_dump(bytes: &[u8]) {
    hex_dump_with_options(bytes, &HexDumpOptions::default())
}

pub(crate) fn hex_dump_with_options(bytes: &[u8], options: &HexDumpOptions) {
    let columns = options.columns.unwrap_or(32);
    let mut i = 0;
    let n = bytes.len();
    while i < n {
        let bound = n.min(i + columns);
        for j in i..bound {
            if j > i {
                print!(" ");
            }
            print!("{byte:02X}", byte = bytes[j])
        }
        println!();
        i += columns;
    }
}
