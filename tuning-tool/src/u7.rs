macro_rules! u7_newtype {
    ($ident: ident, $vis: vis) => {
        #[derive(Clone, Copy, Debug, PartialEq, tuning_tool_derive::U7)]
        $vis struct $ident(u8);

    };
}
pub(crate) use u7_newtype;
