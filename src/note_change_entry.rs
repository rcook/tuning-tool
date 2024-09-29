use crate::mts_entry::MtsEntry;
use midly::num::u7;

#[derive(Clone, Debug)]
pub(crate) struct NoteChangeEntry {
    pub(crate) kk: u7,
    pub(crate) mts: MtsEntry,
}
