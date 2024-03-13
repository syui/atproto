#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let response = client
        .app_bsky_notification_get_unread_count()
        .seen_at(chrono::Utc::now())
        .await
        .unwrap();
    println!("{:#?}", response);
}