#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let response = client.com_atproto_server_list_app_passwords().await.unwrap();
    println!("{:#?}", response);
}