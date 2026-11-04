use std::fmt::{Display, Formatter, Result as FmtResult};

pub(crate) const BETA: &str = "beta";

#[derive(Debug, PartialEq, Eq)]
pub struct Entry {
    pub(crate) link: Link,
    pub(crate) status: Option<Status>,
    pub kind: String,
    pub version: String,
    pub(crate) description: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Link {
    pub(crate) text: String,
    pub(crate) url: String,
}

/// `<span class="pill pill-new">NEW</span>`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    New,
    Beta,
}

impl Status {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::New => "new",
            Self::Beta => BETA,
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.as_str())
    }
}
