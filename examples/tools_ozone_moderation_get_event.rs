#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let id = 1;
    let response = client.tools_ozone_moderation_get_event(id).await.unwrap();
    println!("{:#?}", response);
}