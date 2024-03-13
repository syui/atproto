#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let did = "your did";
    let password = "your password";
    let token = "your token";
    let response = client
        .com_atproto_server_delete_account(did, password, token)
        .await
        .unwrap();
    println!("{:#?}", response);
}