#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let response = client.tools_ozone_communication_list_templates().await.unwrap();
    println!("{:#?}", response);
}