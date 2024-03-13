#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let response = client
        .app_bsky_actor_search_actors_typeahead()
        .limit(1)
        .q("your q")
        .term("your term")
        .await
        .unwrap();
    println!("{:#?}", response);
}