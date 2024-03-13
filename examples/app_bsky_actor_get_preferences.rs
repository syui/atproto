#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let response = client.app_bsky_actor_get_preferences().await.unwrap();
    println!("{:#?}", response);
}