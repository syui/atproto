#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let reason_type = ComAtprotoModerationDefsReasonType(serde_json::json!({}));
    let subject = serde_json::json!({});
    let response = client
        .com_atproto_moderation_create_report(reason_type, subject)
        .reason("your reason")
        .await
        .unwrap();
    println!("{:#?}", response);
}