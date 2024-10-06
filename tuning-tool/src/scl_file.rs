use crate::fs::read_to_string_lossy;
use crate::interval::Interval;
use crate::scale::Scale;
use anyhow::{bail, Error, Result};
use log::trace;
use std::path::Path;
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Debug)]
pub(crate) struct SclFile {
    file_name: Option<String>,
    description: String,
    scale: Scale,
}

impl SclFile {
    pub(crate) fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        trace!("Reading .scl file {path}", path = path.as_ref().display());
        read_to_string_lossy(path)?.parse()
    }

    #[allow(unused)]
    pub(crate) const fn file_name(&self) -> &Option<String> {
        &self.file_name
    }

    #[allow(unused)]
    pub(crate) fn description(&self) -> &str {
        self.description.as_str()
    }

    pub(crate) fn scale(&self) -> &Scale {
        &self.scale
    }
}

impl FromStr for SclFile {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        trace!("Content is [[{s}]]");

        let mut lines = s.lines().map(|line| line.trim()).peekable();

        let Some(line) = lines.peek() else {
            bail!("Invalid tuning string")
        };

        let file_name = if line.starts_with("!") && line.ends_with(".scl") {
            let file_name = line[1..].trim();
            _ = lines.next().expect("Consume line");
            trace!("Parsed file name {file_name}");
            Some(String::from(file_name))
        } else {
            trace!("Parsed no file name");
            None
        };

        // Now skip all comment lines
        let mut lines = lines.filter(|line| !line.starts_with("!"));

        let Some(description) = lines.next() else {
            bail!("No description found")
        };

        trace!(
            "Parsed description {description}",
            description = if description.is_empty() {
                "(empty)"
            } else {
                description
            }
        );

        // Now skip all blank lines too
        let mut lines = lines.filter(|line| !line.is_empty());

        let Some(interval_count_str) = lines.next() else {
            bail!("No interval count found")
        };

        let interval_count = interval_count_str.parse::<usize>()?;

        trace!("Parsed interval count {interval_count}");

        let intervals = lines
            .map(|line| {
                let interval = line.parse::<Interval>()?;
                trace!("Parse interval {interval}");
                Ok(interval)
            })
            .collect::<Result<Vec<_>>>()?;
        if intervals.len() != interval_count {
            bail!("Incorrect number of notes")
        }

        Ok(Self {
            file_name,
            description: String::from(description),
            scale: Scale::new(intervals)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::resources::RESOURCE_DIR;
    use crate::scl_file::SclFile;
    use anyhow::{anyhow, Result};
    use include_dir::File;
    use std::ffi::OsStr;

    #[test]
    fn scala_archive() -> Result<()> {
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
            test_scala_file(file)?;
        }

        Ok(())
    }

    #[test]
    fn blank_description() -> Result<()> {
        test_scala_file(
            RESOURCE_DIR
                .get_file("weird.scl")
                .ok_or_else(|| anyhow!("Could not read file"))?,
        )
    }

    fn test_scala_file(file: &File) -> Result<()> {
        let s = String::from_utf8_lossy(file.contents());
        let scala_file = s.parse::<SclFile>()?;
        let file_name = scala_file.file_name();
        assert!(file_name.is_some() || file_name.is_none());
        Ok(())
    }
}
