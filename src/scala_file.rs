use crate::fs::read_to_string_lossy;
use crate::interval::Interval;
use crate::scale::Scale;
use anyhow::{bail, Error, Result};
use std::path::Path;
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Debug)]
pub(crate) struct ScalaFile {
    file_name: Option<String>,
    description: String,
    scale: Scale,
}

impl ScalaFile {
    pub(crate) fn read(path: &Path) -> Result<Self> {
        read_to_string_lossy(path)?.parse()
    }

    pub(crate) const fn file_name(&self) -> &Option<String> {
        &self.file_name
    }

    pub(crate) fn description(&self) -> &str {
        self.description.as_str()
    }

    pub(crate) fn scale(&self) -> &Scale {
        &self.scale
    }

    pub(crate) fn dump(&self) {
        if let Some(file_name) = &self.file_name {
            println!("File name: {file_name}");
        }

        println!("Description: {description}", description = self.description);

        println!("Steps: {step_count}", step_count = self.scale.step_count());
        println!(
            "Intervals: {interval_count}",
            interval_count = self.scale.interval_count()
        );

        for (i, note) in self.scale.intervals().iter().enumerate() {
            println!("(note {i}): {cents}", cents = note.cents());
        }
    }
}

impl FromStr for ScalaFile {
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

        let interval_count = count_str.parse::<usize>()? + 1;

        let mut intervals = Vec::with_capacity(interval_count);
        intervals.push(Interval::unison());

        for line in lines {
            intervals.push(line.parse()?);
        }

        if intervals.len() != interval_count {
            bail!("Incorrect number of notes")
        }

        Ok(Self {
            file_name,
            description: String::from(description),
            scale: Scale::new(intervals),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::scale::Scale;
    use crate::{resources::RESOURCE_DIR, scala_file::ScalaFile};
    use anyhow::{anyhow, Result};
    use std::{borrow::Borrow, ffi::OsStr};

    #[test]
    fn scala_archive() -> Result<()> {
        fn test_scala_file(s: &str) -> Result<()> {
            let scala_file = s.parse::<ScalaFile>()?;
            let file_name = scala_file.file_name();
            assert!(file_name.is_some() || file_name.is_none());
            let _ = scala_file.description();
            let scale = scala_file.scale();
            let step_count = scale.step_count();
            let interval_count = scale.interval_count();
            assert_eq!(interval_count, step_count + 1);
            assert_eq!(interval_count, scale.intervals().len());
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
            test_scala_file(s.borrow())?;
        }

        Ok(())
    }
}
