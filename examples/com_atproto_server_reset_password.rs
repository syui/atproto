#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let password = "your password";
    let token = "your token";
    let response = client
        .com_atproto_server_reset_password(password, token)
        .await
        .unwrap();
    println!("{:#?}", response);
}