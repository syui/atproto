#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let list = "your list";
    let response = client.app_bsky_graph_unmute_actor_list(list).await.unwrap();
    println!("{:#?}", response);
}