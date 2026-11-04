use std::fmt::{Display, Formatter, Result as FmtResult};

use super::HEADING;
use crate::{
    entry::{BETA, Entry, Link, Status},
    marker::Marker,
};

const BACKTICK: Marker = Marker::new("`", "`");
const LINK_TEXT: Marker = Marker::new("[", "]");
const PAREN: Marker = Marker::new("(", ")");

impl Entry {
    /// ```text
    /// ### `kind` v1 [name](url)
    ///
    /// description.
    /// ```
    pub(crate) fn parse_md(block: &[&str]) -> Result<Self, String> {
        let (heading, rest) = HEADING.split_title(block)?;
        let [description] = rest else {
            return Err(format!(
                "{heading:?} must have one description line: {rest:?}"
            ));
        };
        let malformed = || format!("a malformed heading: {heading:?}");

        let (kind, rest) = BACKTICK.take(heading).ok_or_else(malformed)?;
        let (version, rest) = rest.trim_start().split_once(' ').ok_or_else(malformed)?;
        let (text, rest) = LINK_TEXT.take(rest).ok_or_else(malformed)?;
        let (url, rest) = PAREN.take(rest).ok_or_else(malformed)?;
        let status = match rest.trim_start() {
            "" => None,
            rest => Some(Status::parse_md(PAREN.strip(rest).ok_or_else(malformed)?)?),
        };

        Ok(Self {
            link: Link {
                text: text.to_owned(),
                url: url.to_owned(),
            },
            status,
            kind: kind.to_owned(),
            version: VersionLabel::parse_md(version).0.to_owned(),
            description: description.to_string(),
        })
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "### `{}` {} {}",
            self.kind,
            VersionLabel(&self.version),
            self.link
        )?;
        if let Some(status) = self.status {
            write!(f, " ({status})")?;
        }
        write!(f, "\n\n{}", self.description)
    }
}

/// `v1`, `v2`, `beta`
struct VersionLabel<'a>(&'a str);

impl<'a> VersionLabel<'a> {
    fn parse_md(label: &'a str) -> Self {
        match label.strip_prefix('v') {
            Some(number) if is_number(number) => Self(number),
            _ => Self(label),
        }
    }
}

impl Display for VersionLabel<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if is_number(self.0) {
            write!(f, "v{}", self.0)
        } else {
            f.write_str(self.0)
        }
    }
}

fn is_number(text: &str) -> bool {
    !text.is_empty() && text.bytes().all(|b| b.is_ascii_digit())
}

impl Display for Link {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "[{}]({})", self.text, self.url)
    }
}

impl Status {
    pub(crate) fn parse_md(text: &str) -> Result<Self, String> {
        match text {
            "new" => Ok(Self::New),
            BETA => Ok(Self::Beta),
            _ => Err(format!("an unknown status: {text:?}")),
        }
    }
}
