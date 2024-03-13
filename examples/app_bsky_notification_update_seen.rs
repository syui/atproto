#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let seen_at = chrono::Utc::now();
    let response = client.app_bsky_notification_update_seen(seen_at).await.unwrap();
    println!("{:#?}", response);
}