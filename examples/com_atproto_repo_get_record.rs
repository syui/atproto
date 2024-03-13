#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let collection = "your collection";
    let repo = "your repo";
    let rkey = "your rkey";
    let response = client
        .com_atproto_repo_get_record(collection, repo, rkey)
        .cid("your cid")
        .await
        .unwrap();
    println!("{:#?}", response);
}