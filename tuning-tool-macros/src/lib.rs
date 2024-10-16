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

use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(U7)]
pub fn u7_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { vis, ident, .. } = parse_macro_input!(input);
    let panic_message = Literal::string(&format!("Invalid {} constant", ident));
    let output = quote! {
        impl #ident {
            #vis const ZERO: Self = Self::constant::<0>();
            #vis const ONE: Self = Self::constant::<1>();
            #vis const MIN: Self = Self::constant::<0>();
            #vis const MAX: Self = Self::constant::<127>();

            const MASK: u8 = 0x7f;

            #vis const fn constant<const N: u8>() -> Self {
                if N & Self::MASK != N {
                    panic!(#panic_message);
                }
                Self(N)
            }

            #vis const fn from_u8_lossy(value: u8) -> Self {
                Self(value & Self::MASK)
            }

            #vis fn all() -> impl std::iter::Iterator<Item = Self> {
                std::iter::successors(Some(Self::ZERO), |x| x.checked_successor())
            }

            #vis fn to_u8_slice(slice: &[Self]) -> &[u8] {
                unsafe { &*(slice as *const [#ident] as *const [u8]) }
            }

            #vis fn is_min(&self) -> bool {
                self.0 == Self::MIN.0
            }

            #vis fn is_max(&self) -> bool {
                self.0 == Self::MAX.0
            }

            #vis const fn to_u8(self) -> u8 {
                self.0
            }

            #vis fn widening_successor(self) -> u8 {
                self.0 + 1
            }

            #vis fn widening_predecessor(self) -> i8 {
                self.0 as i8 - 1
            }

            #vis fn checked_successor(self) -> Option<Self> {
                if self.0 >= Self::MASK {
                    None
                } else {
                    Some(Self(self.0 + 1))
                }
            }

            #vis fn checked_predecessor(self) -> Option<Self> {
                if self.0 > 0 {
                    Some(Self(self.0 - 1))
                } else {
                    None
                }
            }

            #vis fn widening_add(self, rhs: Self) -> u8 {
                self.0 + rhs.0
            }

            #vis fn widening_sub(self, rhs: Self) -> i8 {
                self.0 as i8 - rhs.0 as i8
            }

            #vis fn checked_add(self, rhs: Self) -> Option<Self> {
                let result = self.0.checked_add(rhs.0)?;
                if result > Self::MASK {
                    None
                } else {
                    Some(Self(result))
                }
            }

            #vis fn checked_sub(self, rhs: Self) -> Option<Self> {
                let result = self.0.checked_sub(rhs.0)?;
                if result > Self::MASK {
                    None
                } else {
                    Some(Self(result))
                }
            }

            #vis fn up_to(self, end: Self) -> Option<impl std::iter::Iterator<Item = Self>> {
                _ = end.checked_sub(self)?;
                Some(std::iter::successors(Some(self), move |x| {
                    if x.0 < end.0 {
                        Some(Self(x.0 + 1))
                    } else {
                        None
                    }
                }))
            }
        }

        impl std::convert::TryFrom<u8> for #ident {
            type Error = tuning_tool_lib::error::TryFromU8Error;

            fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
                if value > Self::MASK {
                    Err(Self::Error::OutOfRange(value))
                } else {
                    Ok(Self(value))
                }
            }
        }

        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::fmt::LowerHex for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::LowerHex::fmt(&self.0, f)
            }
        }

        impl std::fmt::UpperHex for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::UpperHex::fmt(&self.0, f)
            }
        }

        impl std::str::FromStr for #ident {
            type Err = tuning_tool_lib::error::FromStrError;

            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                let value = s.parse().map_err(|e| Self::Err::Other(e))?;
                if value > Self::MASK {
                    Err(Self::Err::OutOfRange(value))
                } else {
                    Ok(Self(value))
                }
            }
        }

        impl tuning_tool_lib::u7::U7 for #ident {
            const ZERO: Self = Self::ZERO;

            fn to_u8(self) -> u8 {
                Self::to_u8(self)
            }
        }
    };
    output.into()
}

#[proc_macro]
pub fn scale(input: TokenStream) -> TokenStream {
    fn make_literals(input: &TokenStream) -> Vec<String> {
        let mut strs = Vec::new();
        let mut iter = input.clone().into_iter().peekable();
        while let Some(tt) = iter.next() {
            if iter.next_if(|s| s.to_string() == "/").is_some() {
                let denom_tt = iter.next().expect("Malformed ratio?");
                strs.push(format!("{tt}/{denom_tt}"));
            } else {
                strs.push(tt.to_string())
            }
        }
        strs
    }

    let parse_exprs = make_literals(&input)
        .iter()
        .map(|s| {
            quote! {
                #s.parse::<crate::interval::Interval>()?
            }
        })
        .collect::<Vec<_>>();

    let output = quote! {{
        fn inner() -> anyhow::Result<Vec<crate::interval::Interval>> {
            Ok(vec![#(#parse_exprs),*])
        }
        inner().and_then(|intervals| crate::scale::Scale::new(intervals)).expect("Must be a valid scale")
    }};

    output.into()
}
