#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let name = "your name";
    let response = client.com_atproto_server_create_app_password(name).await.unwrap();
    println!("{:#?}", response);
}