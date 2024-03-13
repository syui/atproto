#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let collection = "your collection";
    let repo = "your repo";
    let response = client
        .com_atproto_repo_list_records(collection, repo)
        .cursor("your cursor")
        .limit(1)
        .reverse(true)
        .rkey_end("your rkey end")
        .rkey_start("your rkey start")
        .await
        .unwrap();
    println!("{:#?}", response);
}