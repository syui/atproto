#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let collection = "your collection";
    let record = serde_json::json!({});
    let repo = "your repo";
    let response = client
        .com_atproto_repo_create_record(collection, record, repo)
        .rkey("your rkey")
        .swap_commit("your swap commit")
        .validate(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}