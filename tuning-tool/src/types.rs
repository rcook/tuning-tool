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

f64_newtype!(Ratio, pub(crate));

u7_newtype!(Char7, pub(crate));
u7_newtype!(Checksum, pub(crate));
u7_newtype!(ChunkSize, pub(crate));
u7_newtype!(DeviceId, pub(crate));
u7_newtype!(KeyNumber, pub(crate));
u7_newtype!(Lsb, pub(crate));
u7_newtype!(MidiValue, pub(crate));
u7_newtype!(Msb, pub(crate));
u7_newtype!(Preset, pub(crate));
