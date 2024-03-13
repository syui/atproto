#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let aud = "your aud";
    let response = client.com_atproto_server_get_service_auth(aud).await.unwrap();
    println!("{:#?}", response);
}