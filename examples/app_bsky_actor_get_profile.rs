#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let actor = "your actor";
    let response = client.app_bsky_actor_get_profile(actor).await.unwrap();
    println!("{:#?}", response);
}