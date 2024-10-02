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
