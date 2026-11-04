use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, Copy)]
pub struct Marker {
    pub open: &'static str,
    pub close: &'static str,
}

pub struct Wrapped<T> {
    marker: Marker,
    inner: T,
}

impl Marker {
    pub const fn new(open: &'static str, close: &'static str) -> Self {
        Self { open, close }
    }

    pub fn wrap<T: Display>(self, inner: T) -> Wrapped<T> {
        Wrapped {
            marker: self,
            inner,
        }
    }

    pub fn strip(self, line: &str) -> Option<&str> {
        line.strip_prefix(self.open)?.strip_suffix(self.close)
    }

    pub fn take(self, text: &str) -> Option<(&str, &str)> {
        text.strip_prefix(self.open)?.split_once(self.close)
    }

    pub fn split_blocks<'s, 'a>(self, lines: &'s [&'a str]) -> (&'s [&'a str], Vec<&'s [&'a str]>) {
        let start = lines
            .iter()
            .position(|line| line.starts_with(self.open))
            .unwrap_or(lines.len());
        let (before, blocks) = lines.split_at(start);

        (
            before,
            blocks
                .chunk_by(|_, next| !next.starts_with(self.open))
                .collect(),
        )
    }

    pub fn split_title<'s, 'a>(
        self,
        lines: &'s [&'a str],
    ) -> Result<(&'a str, &'s [&'a str]), String> {
        let [line, rest @ ..] = lines else {
            return Err("an empty block".to_owned());
        };
        let text = self
            .strip(line)
            .ok_or_else(|| format!("a malformed title: {line:?}"))?;

        Ok((text, rest))
    }
}

impl<T: Display> Display for Wrapped<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}{}{}", self.marker.open, self.inner, self.marker.close)
    }
}
