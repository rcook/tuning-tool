use crate::mts_entry::MtsEntry;
use crate::types::KeyNumber;

#[derive(Clone, Debug)]
pub(crate) struct NoteChangeEntry {
    pub(crate) key_number: KeyNumber,
    pub(crate) mts: MtsEntry,
}
