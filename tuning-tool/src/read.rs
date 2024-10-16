// Copyright (c) 2024 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

use anyhow::Result;
use tuning_tool_lib::u7::U7;

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
