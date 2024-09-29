use derive_builder::Builder;
use std::default;
use std::ops::Rem;

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

    for (i, b) in bytes.iter().enumerate() {
        let column = i.rem(columns);
        if column > 0 {
            print!(" ");
        }
        print!("{b:02X}");
        if column == columns - 1 {
            println!();
        }
    }

    if bytes.len().rem(columns) > 0 {
        println!();
    }
}
