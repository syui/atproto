#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let response = client
        .app_bsky_unspecced_get_popular()
        .cursor("your cursor")
        .include_nsfw(true)
        .limit(1)
        .await
        .unwrap();
    println!("{:#?}", response);
}