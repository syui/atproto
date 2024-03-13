#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let repo = "your repo";
    let writes = vec![serde_json::json!({})];
    let response = client
        .com_atproto_repo_apply_writes(repo, writes)
        .swap_commit("your swap commit")
        .validate(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}