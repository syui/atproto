#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let cids = &["your cids"];
    let did = "your did";
    let response = client.com_atproto_sync_get_blocks(cids, did).await.unwrap();
    println!("{:#?}", response);
}