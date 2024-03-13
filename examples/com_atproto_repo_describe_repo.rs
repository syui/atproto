#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let repo = "your repo";
    let response = client.com_atproto_repo_describe_repo(repo).await.unwrap();
    println!("{:#?}", response);
}