use anyhow::Result;
use tuning_tool_lib::U7;

pub(crate) fn read<U, I>(iter: &mut I) -> Result<U>
where
    U: U7,
    I: Iterator<Item = u8>,
{
    let byte: u8 = iter
        .next()
        .ok_or_else(|| ::anyhow::anyhow!("Failed to read byte"))?;
    Ok(byte.try_into()?)
}

pub(crate) fn read_multi<U, I, const N: usize>(iter: &mut I) -> Result<[U; N]>
where
    U: U7,
    I: Iterator<Item = u8>,
{
    let mut result = [U::ZERO; N];
    for blah in result.iter_mut() {
        *blah = read::<U, I>(iter)?;
    }
    Ok(result)
}
