#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let collection = "your collection";
    let did = "your did";
    let rkey = "your rkey";
    let response = client
        .com_atproto_sync_get_record(collection, did, rkey)
        .commit("your commit")
        .await
        .unwrap();
    println!("{:#?}", response);
}