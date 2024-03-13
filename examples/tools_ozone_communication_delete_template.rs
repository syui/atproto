#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let id = "your id";
    let response = client.tools_ozone_communication_delete_template(id).await.unwrap();
    println!("{:#?}", response);
}