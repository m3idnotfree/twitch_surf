use std::collections::BTreeSet;

use twitch_docs::{
    snapshot::Table,
    source::{Variant, update},
};
use twitch_surf::SubscriptionType;

const SNAPSHOT: &str = include_str!("../docs/twitch-subscription-types.md");

fn snapshot() -> Table {
    SNAPSHOT.parse().unwrap()
}

#[test]
fn matching_snapshot_format() {
    assert!(
        snapshot().to_string() == SNAPSHOT,
        "`Table` did not round-trip the snapshot"
    );
}

#[test]
fn matching_subscription_types() {
    let table = snapshot();
    let snapshot: BTreeSet<(&str, &str)> = table
        .entries()
        .iter()
        .map(|entry| (entry.kind.as_str(), entry.version.as_str()))
        .collect();

    let ours: BTreeSet<(&str, &str)> = SubscriptionType::ALL
        .iter()
        .map(|ty| (ty.as_str(), ty.version()))
        .collect();
    let missing: Vec<_> = snapshot.difference(&ours).collect();
    let removed: Vec<_> = ours.difference(&snapshot).collect();

    assert!(
        missing.is_empty() && removed.is_empty(),
        "missing: {missing:?}\n\
        removed: {removed:?}"
    );
}

#[test]
fn matching_docs() {
    let source = include_str!("../src/subscription_type.rs");

    assert!(
        update(source, &snapshot()) == source,
        "run `cargo run -p twitch_docs -- YYYY-MM-DD`"
    );
}

#[test]
fn matching_variant_name() {
    for ty in SubscriptionType::ALL {
        assert_eq!(
            format!("{ty:?}"),
            Variant::new(ty.as_str(), ty.version()).name()
        );
    }
}

#[test]
fn variant_name() {
    assert_eq!(
        Variant::new("automod.message.hold", "1").name(),
        "AutomodMessageHold"
    );
    assert_eq!(
        Variant::new("automod.message.hold", "2").name(),
        "AutomodMessageHoldV2"
    );
    assert_eq!(
        Variant::new("channel.guest_star_session.begin", "beta").name(),
        "ChannelGuestStarSessionBegin"
    );
}
