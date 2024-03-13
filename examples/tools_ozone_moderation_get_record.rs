#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let uri = "your uri";
    let response = client
        .tools_ozone_moderation_get_record(uri)
        .cid("your cid")
        .await
        .unwrap();
    println!("{:#?}", response);
}