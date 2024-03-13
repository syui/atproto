#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let created_by = "your created by";
    let event = serde_json::json!({});
    let subject = serde_json::json!({});
    let response = client
        .tools_ozone_moderation_emit_event(created_by, event, subject)
        .subject_blob_cids(&["your subject blob cids"])
        .await
        .unwrap();
    println!("{:#?}", response);
}