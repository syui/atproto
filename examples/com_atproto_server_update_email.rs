#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let email = "your email";
    let response = client
        .com_atproto_server_update_email(email)
        .token("your token")
        .await
        .unwrap();
    println!("{:#?}", response);
}