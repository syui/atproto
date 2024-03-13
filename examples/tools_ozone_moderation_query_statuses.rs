#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let response = client
        .tools_ozone_moderation_query_statuses()
        .appealed(true)
        .comment("your comment")
        .cursor("your cursor")
        .exclude_tags(&["your exclude tags"])
        .ignore_subjects(&["your ignore subjects"])
        .include_muted(true)
        .last_reviewed_by("your last reviewed by")
        .limit(1)
        .reported_after(chrono::Utc::now())
        .reported_before(chrono::Utc::now())
        .review_state("your review state")
        .reviewed_after(chrono::Utc::now())
        .reviewed_before(chrono::Utc::now())
        .sort_direction("your sort direction")
        .sort_field("your sort field")
        .subject("your subject")
        .tags(&["your tags"])
        .takendown(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}