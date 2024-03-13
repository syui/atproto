#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
use atproto::request::AppBskyNotificationRegisterPushRequired;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let args = AppBskyNotificationRegisterPushRequired {
        app_id: "your app id",
        platform: "your platform",
        service_did: "your service did",
        token: "your token",
    };
    let response = client.app_bsky_notification_register_push(args).await.unwrap();
    println!("{:#?}", response);
}