extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(U7)]
pub fn u7_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { vis, ident, .. } = parse_macro_input!(input);
    let iter_ident = Ident::new(&format!("{}Iterator", ident), Span::call_site());
    let output = quote! {
        impl #ident {
            pub const fn new_lossy(value: u8) -> Self {
                Self(value & 0x7f)
            }
        }

        impl foo_lib::u7::U7 for #ident {
            type Iter = #iter_ident;

            const ZERO: #ident = #ident::new_lossy(0);
            const ONE: #ident = #ident::new_lossy(1);
            const MIN: #ident = #ident::new_lossy(0);
            const MAX: #ident = #ident::new_lossy(127);

            fn all() -> Self::Iter {
                #iter_ident::new(0, 127)
            }

            fn as_u8(self) -> u8 {
                self.0
            }

            fn widening_succ(self) -> u8 {
                self.0 + 1
            }

            fn widening_pred(self) -> i8 {
                self.0 as i8 - 1
            }

            fn checked_succ(self) -> Option<Self> {
                if self.0 >= 0x7f {
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
                if result > 0x7f {
                    None
                } else {
                    Some(Self(result))
                }
            }

            fn checked_sub(self, rhs: Self) -> Option<Self> {
                let result = self.0.checked_sub(rhs.0)?;
                if result > 0x7f {
                    None
                } else {
                    Some(Self(result))
                }
            }

            fn iter_up_to(self, end: Self) -> Option<Self::Iter> {
                _ = end.checked_sub(self)?;
                Some(#iter_ident::new(self.0, end.0))
            }
        }

        impl std::convert::TryFrom<u8> for #ident {
            type Error = foo_lib::error::TryFromU8Error;

            fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
                if value > 0x7f {
                    Err(Self::Error::OutOfRange(value))
                } else {
                    Ok(Self(value))
                }
            }
        }

        impl std::str::FromStr for #ident {
            type Err = foo_lib::error::FromStrError;

            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                let value = s.parse().map_err(|e| Self::Err::Other(e))?;
                if value > 0x7f {
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
                    Some(#ident::new_lossy(value))
                } else {
                    None
                }
            }
        }
    };
    output.into()
}
