#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let response = client
        .app_bsky_notification_list_notifications()
        .cursor("your cursor")
        .limit(1)
        .seen_at(chrono::Utc::now())
        .await
        .unwrap();
    println!("{:#?}", response);
}