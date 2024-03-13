#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let id = "your id";
    let response = client
        .tools_ozone_communication_update_template(id)
        .content_markdown("your content markdown")
        .disabled(true)
        .name("your name")
        .subject("your subject")
        .updated_by("your updated by")
        .await
        .unwrap();
    println!("{:#?}", response);
}