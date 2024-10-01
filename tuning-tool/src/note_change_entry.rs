use crate::key_number::KeyNumber;
use crate::mts_entry::MtsEntry;

#[derive(Clone, Debug)]
pub(crate) struct NoteChangeEntry {
    pub(crate) key_number: KeyNumber,
    pub(crate) mts: MtsEntry,
}
