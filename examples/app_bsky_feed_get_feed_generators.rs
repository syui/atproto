#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let feeds = &["your feeds"];
    let response = client.app_bsky_feed_get_feed_generators(feeds).await.unwrap();
    println!("{:#?}", response);
}