#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let did = "your did";
    let response = client.tools_ozone_moderation_get_repo(did).await.unwrap();
    println!("{:#?}", response);
}