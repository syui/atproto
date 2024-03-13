#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let handle = "your handle";
    let response = client.com_atproto_identity_resolve_handle(handle).await.unwrap();
    println!("{:#?}", response);
}