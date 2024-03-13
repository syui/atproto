#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let labels = vec![
        ComAtprotoLabelDefsLabel { cid : Some("your cid".to_owned()), cts :
        chrono::Utc::now(), exp : Some(chrono::Utc::now()), neg : Some(true), sig :
        Some("your sig".to_owned()), src : "your src".to_owned(), uri : "your uri"
        .to_owned(), val : "your val".to_owned(), ver : Some(1) }
    ];
    let response = client.app_bsky_unspecced_apply_labels(labels).await.unwrap();
    println!("{:#?}", response);
}