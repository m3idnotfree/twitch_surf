mod entry;

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

use crate::{URL, entry::Entry, marker::Marker};

const NCHECKED: &str = "checked: ";
const NSOURCE: &str = "source: ";
const SECTION: &str = "## Subscription Types";
const HEADING: Marker = Marker::new("### ", "");

#[derive(Debug, PartialEq, Eq)]
pub struct FrontMatter {
    pub checked: String,
    pub source: String,
}

impl FrontMatter {
    pub fn new(checked: impl Into<String>) -> Self {
        Self {
            checked: checked.into(),
            source: URL.to_owned(),
        }
    }

    pub fn parse_md(snapshot: &str) -> Result<(Self, &str), String> {
        let (front, body) = snapshot
            .strip_prefix("---\n")
            .and_then(|rest| rest.split_once("\n---\n"))
            .ok_or("the snapshot has no front matter")?;
        let field = |key: &str| {
            front
                .lines()
                .find_map(|line| line.strip_prefix(key))
                .map(str::to_owned)
                .ok_or_else(|| format!("the front matter has no {key:?}"))
        };

        Ok((
            Self {
                checked: field(NCHECKED)?,
                source: field(NSOURCE)?,
            },
            body,
        ))
    }
}

impl Display for FrontMatter {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "---\n{NCHECKED}{}\n{NSOURCE}{}\n---",
            self.checked, self.source
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Table {
    front_matter: FrontMatter,
    entries: Vec<Entry>,
}

impl Table {
    pub fn new(front_matter: FrontMatter, entries: Vec<Entry>) -> Self {
        Self {
            front_matter,
            entries,
        }
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }
}

impl FromStr for Table {
    type Err = String;

    fn from_str(snapshot: &str) -> Result<Self, Self::Err> {
        let (front_matter, body) = FrontMatter::parse_md(snapshot)?;
        let lines: Vec<&str> = body.lines().filter(|line| !line.is_empty()).collect();

        let (before, blocks) = HEADING.split_blocks(&lines);
        if before != [SECTION] {
            return Err(format!(
                "expected only {SECTION:?} before the entries: {before:?}"
            ));
        }

        let entries = blocks
            .into_iter()
            .map(Entry::parse_md)
            .collect::<Result<_, _>>()?;

        Ok(Self {
            front_matter,
            entries,
        })
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "{}", self.front_matter)?;
        write!(f, "\n{SECTION}\n")?;

        for entry in &self.entries {
            write!(f, "\n{entry}\n")?;
        }

        Ok(())
    }
}
