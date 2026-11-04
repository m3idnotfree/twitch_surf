use std::collections::{BTreeMap, BTreeSet};

use super::{CLOSE, Item, OPEN, Variant, read_block};
use crate::{
    entry::{Entry, Status},
    snapshot::Table,
};

impl Entry {
    fn variant(&self) -> Variant<'_> {
        Variant::new(&self.kind, &self.version)
    }
}

pub fn update(lib: &str, table: &Table) -> String {
    let mut lines = lib.lines().peekable();
    let mut out = Vec::new();

    while let Some(line) = lines.next_if(|&line| line != OPEN) {
        out.push(line.to_owned());
    }
    let open = lines
        .next()
        .unwrap_or_else(|| panic!("no {OPEN:?} ... {CLOSE:?} block"));
    out.push(open.to_owned());

    let items = read_block(&mut lines).unwrap_or_else(|err| panic!("{err}"));
    out.extend(write_block(&items, table));
    out.push(CLOSE.to_owned());

    out.extend(lines.map(str::to_owned));
    out.join("\n") + "\n"
}

fn write_block(items: &[Item], table: &Table) -> Vec<String> {
    let keys: BTreeSet<Variant> = table.entries().iter().map(Entry::variant).collect();

    let mut attributes: BTreeMap<Variant, &[&str]> = BTreeMap::new();
    for item in items {
        let variant = item.variant;
        assert!(
            keys.contains(&variant),
            "{:?} ({:?}, {:?}) is not in the snapshot",
            variant.name(),
            variant.kind,
            variant.version
        );
        attributes.insert(variant, &item.attributes);
    }

    let mut entries: Vec<&Entry> = table.entries().iter().collect();
    entries.sort_by_key(|entry| sort_key(entry.variant()));

    let mut out = Vec::new();
    let mut names = BTreeMap::new();

    for entry in entries {
        let variant = entry.variant();
        if let Some(first) = names.insert(variant.name(), variant) {
            panic!(
                "{:?} is the name of both {:?} and {:?}",
                variant.name(),
                (first.kind, first.version),
                (variant.kind, variant.version)
            );
        }

        push_doc_comment(&mut out, entry);
        for attribute in attributes.get(&variant).copied().unwrap_or_default() {
            out.push(format!("    {attribute}"));
        }
        out.push(format!("    {variant}"));
    }

    out
}

fn sort_key(variant: Variant<'_>) -> (&str, u32, &str) {
    let number = variant.version.parse().unwrap_or(u32::MAX);
    (variant.kind, number, variant.version)
}

/// ```text
/// /// description.
/// ///
/// /// [Name](url) (beta)
/// ```
fn push_doc_comment(out: &mut Vec<String>, entry: &Entry) {
    out.push(format!("    /// {}", entry.description));
    out.push("    ///".to_owned());
    match entry.status {
        Some(Status::Beta) => out.push(format!("    /// {} (beta)", entry.link)),
        _ => out.push(format!("    /// {}", entry.link)),
    }
}
