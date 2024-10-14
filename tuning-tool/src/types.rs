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

macro_rules! f64_newtype {
    ($ident: ident, $vis: vis) => {
        #[derive(Clone, Copy, Debug, PartialEq)]
        $vis struct $ident(pub(crate) f64);

        impl std::fmt::Display for $ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}
pub(crate) use f64_newtype;

macro_rules! u7_newtype {
    ($ident: ident, $vis: vis) => {
        #[derive(Clone, Copy, Debug, PartialEq, tuning_tool_macros::U7)]
        $vis struct $ident(u8);
    };
}
pub(crate) use u7_newtype;

u7_newtype!(Char7, pub(crate));
u7_newtype!(Checksum, pub(crate));
u7_newtype!(ChunkSize, pub(crate));
u7_newtype!(DeviceId, pub(crate));
u7_newtype!(KeyNumber, pub(crate));
u7_newtype!(Lsb, pub(crate));
u7_newtype!(MidiValue, pub(crate));
u7_newtype!(Msb, pub(crate));
u7_newtype!(Preset, pub(crate));
