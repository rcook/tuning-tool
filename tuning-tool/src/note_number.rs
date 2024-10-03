use crate::types::u7_newtype;

u7_newtype!(NoteNumber, pub(crate));

impl NoteNumber {
    pub(crate) const A4: Self = Self::constant::<69>();
}
