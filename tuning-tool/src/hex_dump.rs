use anyhow::{anyhow, Result};
use std::fmt::Write;
use std::ops::Rem;

pub(crate) fn to_hex_dump(bytes: &[u8], columns: Option<usize>) -> Result<String> {
    let columns = columns.unwrap_or(32);
    let bytes_len = bytes.len();
    let mut s = String::new();
    for (i, b) in bytes.iter().enumerate() {
        let column = i.rem(columns);
        if column > 0 {
            write!(s, " ")?;
        }
        write!(s, "{b:02X}")?;
        if column == columns - 1 && i < bytes_len - 1 {
            writeln!(s)?;
        }
    }
    Ok(s)
}

#[allow(unused)]
pub(crate) fn from_hex_dump(s: &str) -> Result<Vec<u8>> {
    s.split_whitespace()
        .map(|t| u8::from_str_radix(t, 16).map_err(|e| anyhow!(e)))
        .collect::<Result<Vec<_>>>()
}

#[cfg(test)]
mod tests {
    use crate::hex_dump::{from_hex_dump, to_hex_dump};
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case("", &[], Some(4))]
    #[case("01", &[1], Some(4))]
    #[case("01 02", &[1, 2], Some(4))]
    #[case("01 02 03", &[1, 2, 3], Some(4))]
    #[case("01 02 03 04", &[1, 2, 3, 4], Some(4))]
    #[case("01 02 03 04\n05", &[1, 2, 3, 4, 5], Some(4))]
    #[case("01 02 03 04\n05 06", &[1, 2, 3, 4, 5, 6], Some(4))]
    #[case("01 02 03 04\n05 06 07", &[1, 2, 3, 4, 5, 6, 7], Some(4))]
    #[case("01 02 03 04\n05 06 07 08", &[1, 2, 3, 4, 5, 6, 7, 8], Some(4))]
    #[case("01 02 03 04\n05 06 07 08\n09", &[1, 2, 3, 4, 5, 6, 7, 8, 9], Some(4))]
    fn basics(
        #[case] expected: &str,
        #[case] input: &[u8],
        #[case] columns: Option<usize>,
    ) -> Result<()> {
        let s = to_hex_dump(input, columns)?;
        assert_eq!(expected, s);

        let bytes = from_hex_dump(&s)?;
        assert_eq!(input, bytes);

        Ok(())
    }
}
