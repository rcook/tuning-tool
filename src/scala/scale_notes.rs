use crate::scala::scale_note::ScaleNote;
use core::slice::Iter;

#[derive(Clone)]
pub(crate) struct ScaleNotes<'a>(Iter<'a, ScaleNote>);

impl<'a> ScaleNotes<'a> {
    pub(crate) fn new(iter: Iter<'a, ScaleNote>) -> Self {
        Self(iter)
    }
}

impl<'a> Iterator for ScaleNotes<'a> {
    type Item = &'a ScaleNote;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
