#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let actor = "your actor";
    let response = client
        .app_bsky_feed_get_author_feed(actor)
        .cursor("your cursor")
        .filter("your filter")
        .limit(1)
        .await
        .unwrap();
    println!("{:#?}", response);
}