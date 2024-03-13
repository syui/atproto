#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let code_count = 1;
    let use_count = 1;
    let response = client
        .com_atproto_server_create_invite_codes(code_count, use_count)
        .for_accounts(&["your for accounts"])
        .await
        .unwrap();
    println!("{:#?}", response);
}