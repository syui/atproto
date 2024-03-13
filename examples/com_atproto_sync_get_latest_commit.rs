#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let did = "your did";
    let response = client.com_atproto_sync_get_latest_commit(did).await.unwrap();
    println!("{:#?}", response);
}