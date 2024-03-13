#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let response = client
        .com_atproto_identity_sign_plc_operation()
        .also_known_as(&["your also known as"])
        .rotation_keys(&["your rotation keys"])
        .services(serde_json::json!({}))
        .token("your token")
        .verification_methods(serde_json::json!({}))
        .await
        .unwrap();
    println!("{:#?}", response);
}