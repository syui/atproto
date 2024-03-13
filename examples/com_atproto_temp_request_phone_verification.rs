#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let phone_number = "your phone number";
    let response = client
        .com_atproto_temp_request_phone_verification(phone_number)
        .await
        .unwrap();
    println!("{:#?}", response);
}