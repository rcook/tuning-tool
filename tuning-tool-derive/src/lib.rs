use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(U7)]
pub fn u7_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { vis, ident, .. } = parse_macro_input!(input);
    let iter_ident = Ident::new(&format!("{}Iterator", ident), Span::call_site());
    let panic_message = Literal::string(&format!("Invalid {} constant", ident));
    let output = quote! {
        impl #ident {
            const MASK: u8 = 0x7f;

            pub const fn constant<const N: u8>() -> Self {
                if N & Self::MASK != N {
                    panic!(#panic_message);
                }
                Self(N)
            }

            pub const fn from_u8_lossy(value: u8) -> Self {
                Self(value & Self::MASK)
            }
        }

        impl tuning_tool_lib::u7::U7 for #ident {
            type Iter = #iter_ident;

            const ZERO: #ident = Self::constant::<0>();
            const ONE: #ident = Self::constant::<1>();
            const MIN: #ident = Self::constant::<0>();
            const MAX: #ident = Self::constant::<127>();

            fn all() -> Self::Iter {
                #iter_ident::new(0, 127)
            }

            fn to_u8(self) -> u8 {
                self.0
            }

            fn widening_succ(self) -> u8 {
                self.0 + 1
            }

            fn widening_pred(self) -> i8 {
                self.0 as i8 - 1
            }

            fn checked_succ(self) -> Option<Self> {
                if self.0 >= Self::MASK {
                    None
                } else {
                    Some(Self(self.0 + 1))
                }
            }

            fn checked_pred(self) -> Option<Self> {
                if self.0 > 0 {
                    Some(Self(self.0 - 1))
                } else {
                    None
                }
            }

            fn widening_add(self, rhs: Self) -> u8 {
                self.0 + rhs.0
            }

            fn widening_sub(self, rhs: Self) -> i8 {
                self.0 as i8 - rhs.0 as i8
            }

            fn checked_add(self, rhs: Self) -> Option<Self> {
                let result = self.0.checked_add(rhs.0)?;
                if result > Self::MASK {
                    None
                } else {
                    Some(Self(result))
                }
            }

            fn checked_sub(self, rhs: Self) -> Option<Self> {
                let result = self.0.checked_sub(rhs.0)?;
                if result > Self::MASK {
                    None
                } else {
                    Some(Self(result))
                }
            }

            fn up_to(self, end: Self) -> Option<Self::Iter> {
                _ = end.checked_sub(self)?;
                Some(#iter_ident::new(self.0, end.0))
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

        #vis struct #iter_ident {
            curr: u8,
            end: u8,
        }

        impl #iter_ident {
            fn new(start: u8, end: u8) -> Self {
                Self { curr: start, end }
            }
        }

        impl Iterator for #iter_ident {
            type Item = #ident;

            fn next(&mut self) -> Option<Self::Item> {
                let value = self.curr;
                if value <= self.end {
                    self.curr += 1;
                    Some(#ident(value))
                } else {
                    None
                }
            }
        }
    };
    output.into()
}
