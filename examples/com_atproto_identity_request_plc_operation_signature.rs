#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let response = client
        .com_atproto_identity_request_plc_operation_signature()
        .await
        .unwrap();
    println!("{:#?}", response);
}