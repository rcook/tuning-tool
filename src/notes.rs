use crate::note::Note;
use core::slice::Iter;

pub(crate) struct Notes<'a>(Iter<'a, Note>);

impl<'a> Notes<'a> {
    pub(crate) fn new(iter: Iter<'a, Note>) -> Self {
        Self(iter)
    }
}

impl<'a> Iterator for Notes<'a> {
    type Item = &'a Note;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
