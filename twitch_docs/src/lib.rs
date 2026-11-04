pub mod entry;
pub mod marker;
pub mod page;
pub mod snapshot;
pub mod source;

pub const URL: &str = "https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/";

pub async fn fetch() -> String {
    reqwest::get(URL)
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .text()
        .await
        .unwrap()
}
