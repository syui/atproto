#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let content_markdown = "your content markdown";
    let name = "your name";
    let subject = "your subject";
    let response = client
        .tools_ozone_communication_create_template(content_markdown, name, subject)
        .created_by("your created by")
        .await
        .unwrap();
    println!("{:#?}", response);
}