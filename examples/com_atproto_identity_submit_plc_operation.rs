#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let operation = serde_json::json!({});
    let response = client
        .com_atproto_identity_submit_plc_operation(operation)
        .await
        .unwrap();
    println!("{:#?}", response);
}