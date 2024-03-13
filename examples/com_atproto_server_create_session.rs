#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let identifier = "your identifier";
    let password = "your password";
    let response = client
        .com_atproto_server_create_session(identifier, password)
        .await
        .unwrap();
    println!("{:#?}", response);
}