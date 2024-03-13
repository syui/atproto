#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
use atproto::request::ComAtprotoRepoPutRecordRequired;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let args = ComAtprotoRepoPutRecordRequired {
        collection: "your collection",
        record: serde_json::json!({}),
        repo: "your repo",
        rkey: "your rkey",
    };
    let response = client
        .com_atproto_repo_put_record(args)
        .swap_commit("your swap commit")
        .swap_record("your swap record")
        .validate(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}