#![allow(unused_imports)]
use atproto::AtprotoClient;
use atproto::model::*;
#[tokio::main]
async fn main() {
    let client = AtprotoClient::from_env();
    let response = client
        .tools_ozone_moderation_query_events()
        .added_labels(&["your added labels"])
        .added_tags(&["your added tags"])
        .comment("your comment")
        .created_after(chrono::Utc::now())
        .created_before(chrono::Utc::now())
        .created_by("your created by")
        .cursor("your cursor")
        .has_comment(true)
        .include_all_user_records(true)
        .limit(1)
        .removed_labels(&["your removed labels"])
        .removed_tags(&["your removed tags"])
        .report_types(&["your report types"])
        .sort_direction("your sort direction")
        .subject("your subject")
        .types(&["your types"])
        .await
        .unwrap();
    println!("{:#?}", response);
}