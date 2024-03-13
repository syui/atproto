#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let feed = "your feed";
    let response = client.app_bsky_feed_get_feed_generator(feed).await.unwrap();
    println!("{:#?}", response);
}