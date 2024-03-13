#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let use_count = 1;
    let response = client
        .com_atproto_server_create_invite_code(use_count)
        .for_account("your for account")
        .await
        .unwrap();
    println!("{:#?}", response);
}