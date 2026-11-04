use std::collections::BTreeMap;

use scraper::{ElementRef, Html, Selector};

use crate::{
    URL,
    entry::{Entry, Link, Status},
};

const COLUMNS: [&str; 4] = ["Subscription Type", "Name", "Version", "Description"];
const CODE: &str = "code.highlighter-rouge";

pub fn scrape(html: &str) -> Vec<Entry> {
    let doc = Html::parse_document(html);
    let table = Selector::parse("#subscription-types + table").unwrap();

    let table = doc
        .select(&table)
        .next()
        .expect("a table right after #subscription-types");

    let th = Selector::parse("thead th").unwrap();
    let headers: Vec<String> = table.select(&th).map(text_of).collect();

    assert_eq!(headers, COLUMNS, "the table header has changed");

    let tr = Selector::parse("tbody tr").unwrap();
    let entries: Vec<Entry> = table.select(&tr).map(Entry::parse_html).collect();

    let mut seen = BTreeMap::new();
    for entry in &entries {
        let key = (entry.kind.as_str(), entry.version.as_str());
        if let Some(first) = seen.insert(key, entry.link.url.as_str()) {
            panic!(
                "{:?} ({}) appears twice: {first:?} and {:?}",
                entry.kind, entry.version, entry.link.url
            );
        }
    }

    entries
}

impl Entry {
    pub(crate) fn parse_html(tr: ElementRef) -> Self {
        let td = Selector::parse("td").unwrap();

        let cells: Vec<_> = tr.select(&td).collect();
        let [first, kind, version, description] = cells[..] else {
            panic!("a subscription type row must have four cells");
        };

        Self {
            link: Link::parse_html(only(first, "a")),
            status: at_most_one(first, "span.pill").map(Status::parse_html),
            kind: text_of(only(kind, CODE)),
            version: text_of(only(version, CODE)),
            description: text_of(description),
        }
    }
}

impl Link {
    pub(crate) fn parse_html(a: ElementRef) -> Self {
        let href = a.value().attr("href").unwrap();

        Self {
            text: text_of(a),
            url: format!("{URL}{href}"),
        }
    }
}

impl Status {
    pub(crate) fn parse_html(span: ElementRef) -> Self {
        let class = span
            .value()
            .classes()
            .find(|class| class.starts_with("pill-"))
            .expect("a pill with a `pill-*` class");

        match class {
            "pill-new" => Self::New,
            "pill-beta" => Self::Beta,
            _ => panic!("an unknown pill: {class:?}"),
        }
    }
}

fn only<'a>(cell: ElementRef<'a>, selector: &str) -> ElementRef<'a> {
    at_most_one(cell, selector)
        .unwrap_or_else(|| panic!("a cell must have {selector:?}: {}", cell.html()))
}

fn at_most_one<'a>(cell: ElementRef<'a>, selector: &str) -> Option<ElementRef<'a>> {
    let parsed = Selector::parse(selector).unwrap();
    let mut found = cell.select(&parsed);
    let first = found.next();

    assert!(
        found.next().is_none(),
        "a cell must have at most one {selector:?}: {}",
        cell.html()
    );

    first
}

fn text_of(element: ElementRef) -> String {
    element.text().collect::<String>().trim().to_owned()
}
