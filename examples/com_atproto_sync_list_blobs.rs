#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let did = "your did";
    let response = client
        .com_atproto_sync_list_blobs(did)
        .cursor("your cursor")
        .limit(1)
        .since("your since")
        .await
        .unwrap();
    println!("{:#?}", response);
}