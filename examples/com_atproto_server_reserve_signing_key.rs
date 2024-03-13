#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let response = client
        .com_atproto_server_reserve_signing_key()
        .did("your did")
        .await
        .unwrap();
    println!("{:#?}", response);
}