#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let repo = "your repo";
    let response = client
        .com_atproto_repo_rebase_repo(repo)
        .swap_commit("your swap commit")
        .await
        .unwrap();
    println!("{:#?}", response);
}