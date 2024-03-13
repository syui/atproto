#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let handle = "your handle";
    let response = client
        .com_atproto_server_create_account(handle)
        .did("your did")
        .email("your email")
        .invite_code("your invite code")
        .password("your password")
        .plc_op(serde_json::json!({}))
        .recovery_key("your recovery key")
        .verification_code("your verification code")
        .verification_phone("your verification phone")
        .await
        .unwrap();
    println!("{:#?}", response);
}