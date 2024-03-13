#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let response = client
        .com_atproto_server_get_account_invite_codes()
        .create_available(true)
        .include_used(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}