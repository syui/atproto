#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let uris = &["your uris"];
    let response = client.app_bsky_feed_get_posts(uris).await.unwrap();
    println!("{:#?}", response);
}