use anyhow::{bail, Error};
use std::result::Result as StdResult;
use std::str::FromStr;

use crate::note::Note;

#[derive(Debug)]
pub(crate) struct Scale {
    file_name: Option<String>,
    description: String,
    notes: Vec<Note>,
}

impl Scale {
    #[allow(unused)]
    #[must_use]
    pub(crate) const fn file_name(&self) -> &Option<String> {
        &self.file_name
    }

    #[allow(unused)]
    #[must_use]
    pub(crate) fn description(&self) -> &str {
        &self.description.as_str()
    }

    #[allow(unused)]
    #[must_use]
    pub(crate) const fn notes(&self) -> &Vec<Note> {
        &self.notes
    }
}

impl FromStr for Scale {
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
            bail!("Invalid scale string")
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

        let count = count_str.parse::<usize>()?;

        let mut notes = Vec::with_capacity(count + 1);
        notes.push(Note::unison());

        for line in lines {
            notes.push(line.parse()?);
        }

        if notes.len() != count + 1 {
            bail!("Incorrect number of notes")
        }

        Ok(Self {
            file_name,
            description: String::from(description),
            notes,
        })
    }
}
