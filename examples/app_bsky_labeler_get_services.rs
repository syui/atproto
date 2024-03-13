#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let dids = &["your dids"];
    let response = client
        .app_bsky_labeler_get_services(dids)
        .detailed(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}