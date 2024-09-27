use crate::approx_eq::ApproxEq;
use crate::consts::{DEFAULT_CENTS_EPSILON, OCTAVE_CENTS, UNISON_CENTS};
use crate::scala::scale_note::ScaleNote;
use crate::scala::scale_notes::ScaleNotes;
use anyhow::{bail, Error};
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Debug)]
pub(crate) struct Tuning {
    file_name: Option<String>,
    description: String,
    note_count: usize,
    notes: Vec<ScaleNote>,
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
    pub(crate) fn notes(&self) -> ScaleNotes {
        ScaleNotes::new(self.notes.iter())
    }

    #[must_use]
    pub(crate) fn is_octave_repeating(&self) -> bool {
        let Some(first_note) = self.notes.first() else {
            return false;
        };

        if !first_note
            .cents()
            .approx_eq_with_epsilon(UNISON_CENTS, DEFAULT_CENTS_EPSILON)
        {
            return false;
        }

        let Some(last_note) = self.notes.last() else {
            return false;
        };

        if !last_note
            .cents()
            .approx_eq_with_epsilon(OCTAVE_CENTS, DEFAULT_CENTS_EPSILON)
        {
            return false;
        }

        true
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
        notes.push(ScaleNote::unison());

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

#[cfg(test)]
mod tests {
    use crate::resources::RESOURCE_DIR;
    use crate::scala::tuning::Tuning;
    use anyhow::{anyhow, Result};
    use std::{borrow::Borrow, ffi::OsStr};

    #[test]
    fn scala_archive() -> Result<()> {
        fn test_scl(s: &str) -> Result<()> {
            let tuning = s.parse::<Tuning>()?;

            let file_name = tuning.file_name();
            assert!(file_name.is_some() || file_name.is_none());

            let _ = tuning.description();

            let step_count = tuning.step_count();

            let note_count = tuning.note_count();
            assert_eq!(note_count, step_count + 1);

            let notes = tuning.notes().collect::<Vec<_>>();
            assert_eq!(note_count, notes.len());
            Ok(())
        }

        let scl_dir = RESOURCE_DIR
            .get_dir("scl")
            .ok_or_else(|| anyhow!("Could not get scl directory"))?;

        let extension = Some(OsStr::new("scl"));
        let files = scl_dir
            .files()
            .filter(|f| f.path().extension() == extension)
            .collect::<Vec<_>>();
        assert!(files.len() > 5000);

        for file in files {
            let s = String::from_utf8_lossy(file.contents());
            test_scl(s.borrow())?;
        }

        Ok(())
    }
}
