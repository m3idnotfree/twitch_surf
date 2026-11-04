//! ```sh
//! cargo run -p twitch_docs -- "$(date +%F)"
//! ```

use std::{env, fs};

use twitch_docs::{
    fetch, page,
    snapshot::{FrontMatter, Table},
    source::update,
};

macro_rules! root {
    ($path:literal) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/../", $path)
    };
}

const PAGE_PATH: &str = root!("docs/twitch-subscription-types-source.html");
const SNAPSHOT_PATH: &str = root!("docs/twitch-subscription-types.md");
const SOURCE_PATH: &str = root!("src/subscription_type.rs");

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let checked = env::args()
        .nth(1)
        .expect("usage: cargo run -p twitch_docs -- YYYY-MM-DD");

    let html = fetch().await;
    fs::write(PAGE_PATH, &html).unwrap();

    let entries = page::scrape(&html);
    let table = Table::new(FrontMatter::new(checked), entries);

    let source = fs::read_to_string(SOURCE_PATH).expect("failed to read src/subscription_type.rs");
    let source = update(&source, &table);

    fs::write(SNAPSHOT_PATH, table.to_string()).unwrap();
    fs::write(SOURCE_PATH, source).unwrap();
}
