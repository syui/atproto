#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let email = "your email";
    let token = "your token";
    let response = client.com_atproto_server_confirm_email(email, token).await.unwrap();
    println!("{:#?}", response);
}