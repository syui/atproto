#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let preferences = AppBskyActorDefsPreferences(vec![serde_json::json!({})]);
    let response = client.app_bsky_actor_put_preferences(preferences).await.unwrap();
    println!("{:#?}", response);
}