#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let uri = "your uri";
    let response = client
        .app_bsky_feed_get_post_thread(uri)
        .depth(1)
        .parent_height(1)
        .await
        .unwrap();
    println!("{:#?}", response);
}