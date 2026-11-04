mod update;

pub use update::update;

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    iter::Peekable,
};

use crate::marker::Marker;

const OPEN: &str = "types! {";
const CLOSE: &str = "}";
const ARROW: &str = " => ";
/// `("kind", "version"),`
const TUPLE: Marker = Marker::new("(", "),");
const QUOTE: Marker = Marker::new("\"", "\"");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Variant<'a> {
    pub(crate) kind: &'a str,
    pub(crate) version: &'a str,
}

impl<'a> Variant<'a> {
    pub fn new(kind: &'a str, version: &'a str) -> Self {
        Self { kind, version }
    }

    pub fn name(&self) -> String {
        let mut words: Vec<&str> = Vec::new();
        for word in self.kind.split(['.', '_']) {
            assert!(!word.is_empty(), "{:?} has an empty word", self.kind);
            if words.last() != Some(&word) {
                words.push(word);
            }
        }

        let mut name: String = words
            .iter()
            .map(|word| {
                let mut chars = word.chars();
                let first = chars.next().unwrap();
                first.to_uppercase().chain(chars).collect::<String>()
            })
            .collect();

        match self.version.parse::<u32>() {
            Ok(1) => {}
            Ok(number) if number >= 2 => name.push_str(&format!("V{number}")),
            _ if self.version == "beta" => {}
            _ => panic!("{:?} has an unknown version {:?}", self.kind, self.version),
        }

        name
    }

    /// `Name => ("kind", "version"),`
    pub(crate) fn parse_rs(line: &'a str) -> Result<Self, String> {
        let malformed = || format!("a malformed variant: {line:?}");

        let (name, tuple) = line.trim().split_once(ARROW).ok_or_else(malformed)?;
        let fields = TUPLE.strip(tuple).ok_or_else(malformed)?;
        let (kind, rest) = QUOTE.take(fields).ok_or_else(malformed)?;
        let version = rest
            .strip_prefix(", ")
            .and_then(|rest| QUOTE.strip(rest))
            .ok_or_else(malformed)?;

        let variant = Self { kind, version };
        let expected = variant.name();
        if name != expected {
            return Err(format!("{name:?} must be named {expected:?}: {line:?}"));
        }

        Ok(variant)
    }
}

impl Display for Variant<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let fields = format!("{}, {}", QUOTE.wrap(self.kind), QUOTE.wrap(self.version));
        write!(f, "{}{ARROW}{}", self.name(), TUPLE.wrap(fields))
    }
}

struct Item<'a> {
    attributes: Vec<&'a str>,
    variant: Variant<'a>,
}

fn read_block<'a>(
    lines: &mut Peekable<impl Iterator<Item = &'a str>>,
) -> Result<Vec<Item<'a>>, String> {
    let mut items = Vec::new();
    let mut attributes = Vec::new();

    while let Some(line) = lines.next_if(|&line| line != CLOSE) {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }
        if trimmed.starts_with("#[") {
            attributes.push(trimmed);
            continue;
        }

        items.push(Item {
            attributes: std::mem::take(&mut attributes),
            variant: Variant::parse_rs(line)?,
        });
    }

    if lines.next() != Some(CLOSE) {
        return Err(format!("no {OPEN:?} ... {CLOSE:?} block"));
    }
    if !attributes.is_empty() {
        return Err(format!(
            "an attribute with no variant below it: {attributes:?}"
        ));
    }

    Ok(items)
}
