use crate::note::Note;
use crate::notes::Notes;
use anyhow::{bail, Error};
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Debug)]
pub(crate) struct Tuning {
    file_name: Option<String>,
    description: String,
    note_count: usize,
    notes: Vec<Note>,
}

impl Tuning {
    #[must_use]
    pub(crate) const fn file_name(&self) -> &Option<String> {
        &self.file_name
    }

    #[must_use]
    pub(crate) fn description(&self) -> &str {
        self.description.as_str()
    }

    #[must_use]
    pub(crate) const fn step_count(&self) -> usize {
        self.note_count - 1
    }

    #[must_use]
    pub(crate) const fn note_count(&self) -> usize {
        self.note_count
    }

    #[must_use]
    pub(crate) fn notes(&self) -> Notes {
        Notes::new(self.notes.iter())
    }
}

impl FromStr for Tuning {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let mut lines = s
            .lines()
            .filter_map(|line| {
                let s = line.trim();
                if s.is_empty() {
                    None
                } else {
                    Some(s)
                }
            })
            .peekable();

        let Some(line) = lines.peek() else {
            bail!("Invalid tuning string")
        };

        let file_name = match line.strip_prefix("!") {
            Some(suffix) => match suffix.strip_suffix(".scl") {
                Some(prefix) => {
                    _ = lines.next().expect("Consume line");
                    Some(format!("{}.scl", prefix.trim()))
                }
                None => None,
            },
            None => None,
        };

        let mut lines = lines.filter(|line| !line.starts_with("!"));

        let Some(description) = lines.next() else {
            bail!("No description found")
        };

        let Some(count_str) = lines.next() else {
            bail!("No note count found")
        };

        let note_count = count_str.parse::<usize>()? + 1;

        let mut notes = Vec::with_capacity(note_count);
        notes.push(Note::unison());

        for line in lines {
            notes.push(line.parse()?);
        }

        if notes.len() != note_count {
            bail!("Incorrect number of notes")
        }

        Ok(Self {
            file_name,
            description: String::from(description),
            note_count,
            notes,
        })
    }
}
