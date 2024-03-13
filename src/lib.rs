//! [`AtprotoClient`](struct.AtprotoClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
pub use httpclient::{Error, Result, InMemoryResponseExt};
use std::sync::{Arc, OnceLock};
use std::borrow::Cow;
use crate::model::*;
mod serde;
static SHARED_HTTPCLIENT: OnceLock<httpclient::Client> = OnceLock::new();
pub fn default_http_client() -> httpclient::Client {
    httpclient::Client::new()
        .base_url(
            std::env::var("ATPROTO_BASE_URL")
                .expect("Missing environment variable ATPROTO_BASE_URL")
                .as_str(),
        )
}
/// Use this method if you want to add custom middleware to the httpclient.
/// It must be called before any requests are made, otherwise it will have no effect.
/// Example usage:
///
//pub fn init_http_client(init: httpclient::Client) {
//    let _ = SHARED_HTTPCLIENT.set(init);
//}
fn shared_http_client() -> Cow<'static, httpclient::Client> {
    Cow::Borrowed(SHARED_HTTPCLIENT.get_or_init(default_http_client))
}
#[derive(Clone)]
pub struct FluentRequest<'a, T> {
    pub(crate) client: &'a AtprotoClient,
    pub params: T,
}
pub struct AtprotoClient {
    client: Cow<'static, httpclient::Client>,
}
impl AtprotoClient {
    pub fn from_env() -> Self {
        Self {
            client: shared_http_client(),
        }
    }
    pub fn new() -> Self {
        Self {
            client: shared_http_client(),
        }
    }
    pub fn new_with(client: httpclient::Client) -> Self {
    //pub fn new_with(client: httpclient::Client, authentication: AtprotoAuth) -> Self {
        Self {
            client: Cow::Owned(client),
            //authentication,
        }
    }
}
impl AtprotoClient {
    ///Get private preferences attached to the current account. Expected use is synchronization between multiple devices, and import/export during account migration. Requires auth.
    pub fn app_bsky_actor_get_preferences(
        &self,
    ) -> FluentRequest<'_, request::AppBskyActorGetPreferencesRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyActorGetPreferencesRequest {
            },
        }
    }
    ///Get detailed profile view of an actor. Does not require auth, but contains relevant metadata with auth.
    pub fn app_bsky_actor_get_profile(
        &self,
        actor: &str,
    ) -> FluentRequest<'_, request::AppBskyActorGetProfileRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyActorGetProfileRequest {
                actor: actor.to_owned(),
            },
        }
    }
    ///Get detailed profile views of multiple actors.
    pub fn app_bsky_actor_get_profiles(
        &self,
        actors: &[&str],
    ) -> FluentRequest<'_, request::AppBskyActorGetProfilesRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyActorGetProfilesRequest {
                actors: actors.iter().map(|&x| x.to_owned()).collect(),
            },
        }
    }
    ///Get a list of suggested actors. Expected use is discovery of accounts to follow during new account onboarding.
    pub fn app_bsky_actor_get_suggestions(
        &self,
    ) -> FluentRequest<'_, request::AppBskyActorGetSuggestionsRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyActorGetSuggestionsRequest {
                cursor: None,
                limit: None,
            },
        }
    }
    ///Set the private preferences attached to the account.
    pub fn app_bsky_actor_put_preferences(
        &self,
        preferences: AppBskyActorDefsPreferences,
    ) -> FluentRequest<'_, request::AppBskyActorPutPreferencesRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyActorPutPreferencesRequest {
                preferences,
            },
        }
    }
    ///Find actors (profiles) matching search criteria. Does not require auth.
    pub fn app_bsky_actor_search_actors(
        &self,
    ) -> FluentRequest<'_, request::AppBskyActorSearchActorsRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyActorSearchActorsRequest {
                cursor: None,
                limit: None,
                q: None,
                term: None,
            },
        }
    }
    ///Find actor suggestions for a prefix search term. Expected use is for auto-completion during text field entry. Does not require auth.
    pub fn app_bsky_actor_search_actors_typeahead(
        &self,
    ) -> FluentRequest<'_, request::AppBskyActorSearchActorsTypeaheadRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyActorSearchActorsTypeaheadRequest {
                limit: None,
                q: None,
                term: None,
            },
        }
    }
    ///Get a list of feeds (feed generator records) created by the actor (in the actor's repo).
    pub fn app_bsky_feed_get_actor_feeds(
        &self,
        actor: &str,
    ) -> FluentRequest<'_, request::AppBskyFeedGetActorFeedsRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyFeedGetActorFeedsRequest {
                actor: actor.to_owned(),
                cursor: None,
                limit: None,
            },
        }
    }
    ///Get a list of posts liked by an actor. Does not require auth.
    pub fn app_bsky_feed_get_actor_likes(
        &self,
        actor: &str,
    ) -> FluentRequest<'_, request::AppBskyFeedGetActorLikesRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyFeedGetActorLikesRequest {
                actor: actor.to_owned(),
                cursor: None,
                limit: None,
            },
        }
    }
    ///Get a view of an actor's 'author feed' (post and reposts by the author). Does not require auth.
    pub fn app_bsky_feed_get_author_feed(
        &self,
        actor: &str,
    ) -> FluentRequest<'_, request::AppBskyFeedGetAuthorFeedRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyFeedGetAuthorFeedRequest {
                actor: actor.to_owned(),
                cursor: None,
                filter: None,
                limit: None,
            },
        }
    }
    ///Get a hydrated feed from an actor's selected feed generator. Implemented by App View.
    pub fn app_bsky_feed_get_feed(
        &self,
        feed: &str,
    ) -> FluentRequest<'_, request::AppBskyFeedGetFeedRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyFeedGetFeedRequest {
                cursor: None,
                feed: feed.to_owned(),
                limit: None,
            },
        }
    }
    ///Get information about a feed generator. Implemented by AppView.
    pub fn app_bsky_feed_get_feed_generator(
        &self,
        feed: &str,
    ) -> FluentRequest<'_, request::AppBskyFeedGetFeedGeneratorRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyFeedGetFeedGeneratorRequest {
                feed: feed.to_owned(),
            },
        }
    }
    ///Get information about a list of feed generators.
    pub fn app_bsky_feed_get_feed_generators(
        &self,
        feeds: &[&str],
    ) -> FluentRequest<'_, request::AppBskyFeedGetFeedGeneratorsRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyFeedGetFeedGeneratorsRequest {
                feeds: feeds.iter().map(|&x| x.to_owned()).collect(),
            },
        }
    }
    ///Get like records which reference a subject (by AT-URI and CID).
    pub fn app_bsky_feed_get_likes(
        &self,
        uri: &str,
    ) -> FluentRequest<'_, request::AppBskyFeedGetLikesRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyFeedGetLikesRequest {
                cid: None,
                cursor: None,
                limit: None,
                uri: uri.to_owned(),
            },
        }
    }
    ///Get a feed of recent posts from a list (posts and reposts from any actors on the list). Does not require auth.
    pub fn app_bsky_feed_get_list_feed(
        &self,
        list: &str,
    ) -> FluentRequest<'_, request::AppBskyFeedGetListFeedRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyFeedGetListFeedRequest {
                cursor: None,
                limit: None,
                list: list.to_owned(),
            },
        }
    }
    ///Get posts in a thread. Does not require auth, but additional metadata and filtering will be applied for authed requests.
    pub fn app_bsky_feed_get_post_thread(
        &self,
        uri: &str,
    ) -> FluentRequest<'_, request::AppBskyFeedGetPostThreadRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyFeedGetPostThreadRequest {
                depth: None,
                parent_height: None,
                uri: uri.to_owned(),
            },
        }
    }
    ///Gets post views for a specified list of posts (by AT-URI). This is sometimes referred to as 'hydrating' a 'feed skeleton'.
    pub fn app_bsky_feed_get_posts(
        &self,
        uris: &[&str],
    ) -> FluentRequest<'_, request::AppBskyFeedGetPostsRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyFeedGetPostsRequest {
                uris: uris.iter().map(|&x| x.to_owned()).collect(),
            },
        }
    }
    ///Get a list of reposts for a given post.
    pub fn app_bsky_feed_get_reposted_by(
        &self,
        uri: &str,
    ) -> FluentRequest<'_, request::AppBskyFeedGetRepostedByRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyFeedGetRepostedByRequest {
                cid: None,
                cursor: None,
                limit: None,
                uri: uri.to_owned(),
            },
        }
    }
    ///Get a list of suggested feeds (feed generators) for the requesting account.
    pub fn app_bsky_feed_get_suggested_feeds(
        &self,
    ) -> FluentRequest<'_, request::AppBskyFeedGetSuggestedFeedsRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyFeedGetSuggestedFeedsRequest {
                cursor: None,
                limit: None,
            },
        }
    }
    ///Get a view of the requesting account's home timeline. This is expected to be some form of reverse-chronological feed.
    pub fn app_bsky_feed_get_timeline(
        &self,
    ) -> FluentRequest<'_, request::AppBskyFeedGetTimelineRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyFeedGetTimelineRequest {
                algorithm: None,
                cursor: None,
                limit: None,
            },
        }
    }
    ///Find posts matching search criteria, returning views of those posts.
    pub fn app_bsky_feed_search_posts(
        &self,
        q: &str,
    ) -> FluentRequest<'_, request::AppBskyFeedSearchPostsRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyFeedSearchPostsRequest {
                cursor: None,
                limit: None,
                q: q.to_owned(),
            },
        }
    }
    ///Enumerates which accounts the requesting account is currently blocking. Requires auth.
    pub fn app_bsky_graph_get_blocks(
        &self,
    ) -> FluentRequest<'_, request::AppBskyGraphGetBlocksRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyGraphGetBlocksRequest {
                cursor: None,
                limit: None,
            },
        }
    }
    ///Enumerates accounts which follow a specified account (actor).
    pub fn app_bsky_graph_get_followers(
        &self,
        actor: &str,
    ) -> FluentRequest<'_, request::AppBskyGraphGetFollowersRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyGraphGetFollowersRequest {
                actor: actor.to_owned(),
                cursor: None,
                limit: None,
            },
        }
    }
    ///Enumerates accounts which a specified account (actor) follows.
    pub fn app_bsky_graph_get_follows(
        &self,
        actor: &str,
    ) -> FluentRequest<'_, request::AppBskyGraphGetFollowsRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyGraphGetFollowsRequest {
                actor: actor.to_owned(),
                cursor: None,
                limit: None,
            },
        }
    }
    ///Gets a 'view' (with additional context) of a specified list.
    pub fn app_bsky_graph_get_list(
        &self,
        list: &str,
    ) -> FluentRequest<'_, request::AppBskyGraphGetListRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyGraphGetListRequest {
                cursor: None,
                limit: None,
                list: list.to_owned(),
            },
        }
    }
    ///Get mod lists that the requesting account (actor) is blocking. Requires auth.
    pub fn app_bsky_graph_get_list_blocks(
        &self,
    ) -> FluentRequest<'_, request::AppBskyGraphGetListBlocksRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyGraphGetListBlocksRequest {
                cursor: None,
                limit: None,
            },
        }
    }
    ///Enumerates mod lists that the requesting account (actor) currently has muted. Requires auth.
    pub fn app_bsky_graph_get_list_mutes(
        &self,
    ) -> FluentRequest<'_, request::AppBskyGraphGetListMutesRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyGraphGetListMutesRequest {
                cursor: None,
                limit: None,
            },
        }
    }
    ///Enumerates the lists created by a specified account (actor).
    pub fn app_bsky_graph_get_lists(
        &self,
        actor: &str,
    ) -> FluentRequest<'_, request::AppBskyGraphGetListsRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyGraphGetListsRequest {
                actor: actor.to_owned(),
                cursor: None,
                limit: None,
            },
        }
    }
    ///Enumerates accounts that the requesting account (actor) currently has muted. Requires auth.
    pub fn app_bsky_graph_get_mutes(
        &self,
    ) -> FluentRequest<'_, request::AppBskyGraphGetMutesRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyGraphGetMutesRequest {
                cursor: None,
                limit: None,
            },
        }
    }
    ///Enumerates follows similar to a given account (actor). Expected use is to recommend additional accounts immediately after following one account.
    pub fn app_bsky_graph_get_suggested_follows_by_actor(
        &self,
        actor: &str,
    ) -> FluentRequest<'_, request::AppBskyGraphGetSuggestedFollowsByActorRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyGraphGetSuggestedFollowsByActorRequest {
                actor: actor.to_owned(),
            },
        }
    }
    ///Creates a mute relationship for the specified account. Mutes are private in Bluesky. Requires auth.
    pub fn app_bsky_graph_mute_actor(
        &self,
        actor: &str,
    ) -> FluentRequest<'_, request::AppBskyGraphMuteActorRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyGraphMuteActorRequest {
                actor: actor.to_owned(),
            },
        }
    }
    ///Creates a mute relationship for the specified list of accounts. Mutes are private in Bluesky. Requires auth.
    pub fn app_bsky_graph_mute_actor_list(
        &self,
        list: &str,
    ) -> FluentRequest<'_, request::AppBskyGraphMuteActorListRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyGraphMuteActorListRequest {
                list: list.to_owned(),
            },
        }
    }
    ///Unmutes the specified account. Requires auth.
    pub fn app_bsky_graph_unmute_actor(
        &self,
        actor: &str,
    ) -> FluentRequest<'_, request::AppBskyGraphUnmuteActorRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyGraphUnmuteActorRequest {
                actor: actor.to_owned(),
            },
        }
    }
    ///Unmutes the specified list of accounts. Requires auth.
    pub fn app_bsky_graph_unmute_actor_list(
        &self,
        list: &str,
    ) -> FluentRequest<'_, request::AppBskyGraphUnmuteActorListRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyGraphUnmuteActorListRequest {
                list: list.to_owned(),
            },
        }
    }
    ///Get information about a list of labeler services.
    pub fn app_bsky_labeler_get_services(
        &self,
        dids: &[&str],
    ) -> FluentRequest<'_, request::AppBskyLabelerGetServicesRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyLabelerGetServicesRequest {
                detailed: None,
                dids: dids.iter().map(|&x| x.to_owned()).collect(),
            },
        }
    }
    ///Count the number of unread notifications for the requesting account. Requires auth.
    pub fn app_bsky_notification_get_unread_count(
        &self,
    ) -> FluentRequest<'_, request::AppBskyNotificationGetUnreadCountRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyNotificationGetUnreadCountRequest {
                seen_at: None,
            },
        }
    }
    ///Enumerate notifications for the requesting account. Requires auth.
    pub fn app_bsky_notification_list_notifications(
        &self,
    ) -> FluentRequest<'_, request::AppBskyNotificationListNotificationsRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyNotificationListNotificationsRequest {
                cursor: None,
                limit: None,
                seen_at: None,
            },
        }
    }
    ///Register to receive push notifications, via a specified service, for the requesting account. Requires auth.
    pub fn app_bsky_notification_register_push(
        &self,
        args: request::AppBskyNotificationRegisterPushRequired,
    ) -> FluentRequest<'_, request::AppBskyNotificationRegisterPushRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyNotificationRegisterPushRequest {
                app_id: args.app_id.to_owned(),
                platform: args.platform.to_owned(),
                service_did: args.service_did.to_owned(),
                token: args.token.to_owned(),
            },
        }
    }
    ///Notify server that the requesting account has seen notifications. Requires auth.
    pub fn app_bsky_notification_update_seen(
        &self,
        seen_at: chrono::DateTime<chrono::Utc>,
    ) -> FluentRequest<'_, request::AppBskyNotificationUpdateSeenRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyNotificationUpdateSeenRequest {
                seen_at,
            },
        }
    }
    ///Allow a labeler to apply labels directly.
    pub fn app_bsky_unspecced_apply_labels(
        &self,
        labels: Vec<ComAtprotoLabelDefsLabel>,
    ) -> FluentRequest<'_, request::AppBskyUnspeccedApplyLabelsRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyUnspeccedApplyLabelsRequest {
                labels,
            },
        }
    }
    ///DEPRECATED: will be removed soon. Use a feed generator alternative.
    pub fn app_bsky_unspecced_get_popular(
        &self,
    ) -> FluentRequest<'_, request::AppBskyUnspeccedGetPopularRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyUnspeccedGetPopularRequest {
                cursor: None,
                include_nsfw: None,
                limit: None,
            },
        }
    }
    ///An unspecced view of globally popular feed generators.
    pub fn app_bsky_unspecced_get_popular_feed_generators(
        &self,
    ) -> FluentRequest<'_, request::AppBskyUnspeccedGetPopularFeedGeneratorsRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyUnspeccedGetPopularFeedGeneratorsRequest {
                cursor: None,
                limit: None,
                query: None,
            },
        }
    }
    ///Get a list of suggestions (feeds and users) tagged with categories
    pub fn app_bsky_unspecced_get_tagged_suggestions(
        &self,
    ) -> FluentRequest<'_, request::AppBskyUnspeccedGetTaggedSuggestionsRequest> {
        FluentRequest {
            client: self,
            params: request::AppBskyUnspeccedGetTaggedSuggestionsRequest {
            },
        }
    }
    ///Describe the credentials that should be included in the DID doc of an account that is migrating to this service.
    pub fn com_atproto_identity_get_recommended_did_credentials(
        &self,
    ) -> FluentRequest<
        '_,
        request::ComAtprotoIdentityGetRecommendedDidCredentialsRequest,
    > {
        FluentRequest {
            client: self,
            params: request::ComAtprotoIdentityGetRecommendedDidCredentialsRequest {
            },
        }
    }
    ///Request an email with a code to in order to request a signed PLC operation. Requires Auth.
    pub fn com_atproto_identity_request_plc_operation_signature(
        &self,
    ) -> FluentRequest<
        '_,
        request::ComAtprotoIdentityRequestPlcOperationSignatureRequest,
    > {
        FluentRequest {
            client: self,
            params: request::ComAtprotoIdentityRequestPlcOperationSignatureRequest {
            },
        }
    }
    ///Resolves a handle (domain name) to a DID.
    pub fn com_atproto_identity_resolve_handle(
        &self,
        handle: &str,
    ) -> FluentRequest<'_, request::ComAtprotoIdentityResolveHandleRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoIdentityResolveHandleRequest {
                handle: handle.to_owned(),
            },
        }
    }
    ///Signs a PLC operation to update some value(s) in the requesting DID's document.
    pub fn com_atproto_identity_sign_plc_operation(
        &self,
    ) -> FluentRequest<'_, request::ComAtprotoIdentitySignPlcOperationRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoIdentitySignPlcOperationRequest {
                also_known_as: None,
                rotation_keys: None,
                services: None,
                token: None,
                verification_methods: None,
            },
        }
    }
    ///Validates a PLC operation to ensure that it doesn't violate a service's constraints or get the identity into a bad state, then submits it to the PLC registry
    pub fn com_atproto_identity_submit_plc_operation(
        &self,
        operation: serde_json::Value,
    ) -> FluentRequest<'_, request::ComAtprotoIdentitySubmitPlcOperationRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoIdentitySubmitPlcOperationRequest {
                operation,
            },
        }
    }
    ///Updates the current account's handle. Verifies handle validity, and updates did:plc document if necessary. Implemented by PDS, and requires auth.
    pub fn com_atproto_identity_update_handle(
        &self,
        handle: &str,
    ) -> FluentRequest<'_, request::ComAtprotoIdentityUpdateHandleRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoIdentityUpdateHandleRequest {
                handle: handle.to_owned(),
            },
        }
    }
    ///Submit a moderation report regarding an atproto account or record. Implemented by moderation services (with PDS proxying), and requires auth.
    pub fn com_atproto_moderation_create_report(
        &self,
        reason_type: ComAtprotoModerationDefsReasonType,
        subject: serde_json::Value,
    ) -> FluentRequest<'_, request::ComAtprotoModerationCreateReportRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoModerationCreateReportRequest {
                reason: None,
                reason_type,
                subject,
            },
        }
    }
    ///Apply a batch transaction of repository creates, updates, and deletes. Requires auth, implemented by PDS.
    pub fn com_atproto_repo_apply_writes(
        &self,
        repo: &str,
        writes: Vec<serde_json::Value>,
    ) -> FluentRequest<'_, request::ComAtprotoRepoApplyWritesRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoRepoApplyWritesRequest {
                repo: repo.to_owned(),
                swap_commit: None,
                validate: None,
                writes,
            },
        }
    }
    ///Create a single new repository record. Requires auth, implemented by PDS.
    pub fn com_atproto_repo_create_record(
        &self,
        collection: &str,
        record: serde_json::Value,
        repo: &str,
    ) -> FluentRequest<'_, request::ComAtprotoRepoCreateRecordRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoRepoCreateRecordRequest {
                collection: collection.to_owned(),
                record,
                repo: repo.to_owned(),
                rkey: None,
                swap_commit: None,
                validate: None,
            },
        }
    }
    ///Delete a repository record, or ensure it doesn't exist. Requires auth, implemented by PDS.
    pub fn com_atproto_repo_delete_record(
        &self,
        collection: &str,
        repo: &str,
        rkey: &str,
    ) -> FluentRequest<'_, request::ComAtprotoRepoDeleteRecordRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoRepoDeleteRecordRequest {
                collection: collection.to_owned(),
                repo: repo.to_owned(),
                rkey: rkey.to_owned(),
                swap_commit: None,
                swap_record: None,
            },
        }
    }
    ///Get information about an account and repository, including the list of collections. Does not require auth.
    pub fn com_atproto_repo_describe_repo(
        &self,
        repo: &str,
    ) -> FluentRequest<'_, request::ComAtprotoRepoDescribeRepoRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoRepoDescribeRepoRequest {
                repo: repo.to_owned(),
            },
        }
    }
    ///Get a single record from a repository. Does not require auth.
    pub fn com_atproto_repo_get_record(
        &self,
        collection: &str,
        repo: &str,
        rkey: &str,
    ) -> FluentRequest<'_, request::ComAtprotoRepoGetRecordRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoRepoGetRecordRequest {
                cid: None,
                collection: collection.to_owned(),
                repo: repo.to_owned(),
                rkey: rkey.to_owned(),
            },
        }
    }
    ///Import a repo in the form of a CAR file. Requires Content-Length HTTP header to be set.
    pub fn com_atproto_repo_import_repo(
        &self,
    ) -> FluentRequest<'_, request::ComAtprotoRepoImportRepoRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoRepoImportRepoRequest {
            },
        }
    }
    ///Returns a list of missing blobs for the requesting account. Intended to be used in the account migration flow.
    pub fn com_atproto_repo_list_missing_blobs(
        &self,
    ) -> FluentRequest<'_, request::ComAtprotoRepoListMissingBlobsRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoRepoListMissingBlobsRequest {
                cursor: None,
                limit: None,
            },
        }
    }
    ///List a range of records in a repository, matching a specific collection. Does not require auth.
    pub fn com_atproto_repo_list_records(
        &self,
        collection: &str,
        repo: &str,
    ) -> FluentRequest<'_, request::ComAtprotoRepoListRecordsRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoRepoListRecordsRequest {
                collection: collection.to_owned(),
                cursor: None,
                limit: None,
                repo: repo.to_owned(),
                reverse: None,
                rkey_end: None,
                rkey_start: None,
            },
        }
    }
    ///Write a repository record, creating or updating it as needed. Requires auth, implemented by PDS.
    pub fn com_atproto_repo_put_record(
        &self,
        args: request::ComAtprotoRepoPutRecordRequired,
    ) -> FluentRequest<'_, request::ComAtprotoRepoPutRecordRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoRepoPutRecordRequest {
                collection: args.collection.to_owned(),
                record: args.record,
                repo: args.repo.to_owned(),
                rkey: args.rkey.to_owned(),
                swap_commit: None,
                swap_record: None,
                validate: None,
            },
        }
    }
    ///Simple rebase of repo that deletes history
    pub fn com_atproto_repo_rebase_repo(
        &self,
        repo: &str,
    ) -> FluentRequest<'_, request::ComAtprotoRepoRebaseRepoRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoRepoRebaseRepoRequest {
                repo: repo.to_owned(),
                swap_commit: None,
            },
        }
    }
    ///Upload a new blob, to be referenced from a repository record. The blob will be deleted if it is not referenced within a time window (eg, minutes). Blob restrictions (mimetype, size, etc) are enforced when the reference is created. Requires auth, implemented by PDS.
    pub fn com_atproto_repo_upload_blob(
        &self,
    ) -> FluentRequest<'_, request::ComAtprotoRepoUploadBlobRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoRepoUploadBlobRequest {
            },
        }
    }
    ///Activates a currently deactivated account. Used to finalize account migration after the account's repo is imported and identity is setup.
    pub fn com_atproto_server_activate_account(
        &self,
    ) -> FluentRequest<'_, request::ComAtprotoServerActivateAccountRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoServerActivateAccountRequest {
            },
        }
    }
    ///Returns the status of an account, especially as pertaining to import or recovery. Can be called many times over the course of an account migration. Requires auth and can only be called pertaining to oneself.
    pub fn com_atproto_server_check_account_status(
        &self,
    ) -> FluentRequest<'_, request::ComAtprotoServerCheckAccountStatusRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoServerCheckAccountStatusRequest {
            },
        }
    }
    ///Confirm an email using a token from com.atproto.server.requestEmailConfirmation.
    pub fn com_atproto_server_confirm_email(
        &self,
        email: &str,
        token: &str,
    ) -> FluentRequest<'_, request::ComAtprotoServerConfirmEmailRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoServerConfirmEmailRequest {
                email: email.to_owned(),
                token: token.to_owned(),
            },
        }
    }
    ///Create an account. Implemented by PDS.
    pub fn com_atproto_server_create_account(
        &self,
        handle: &str,
    ) -> FluentRequest<'_, request::ComAtprotoServerCreateAccountRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoServerCreateAccountRequest {
                did: None,
                email: None,
                handle: handle.to_owned(),
                invite_code: None,
                password: None,
                plc_op: None,
                recovery_key: None,
                verification_code: None,
                verification_phone: None,
            },
        }
    }
    ///Create an App Password.
    pub fn com_atproto_server_create_app_password(
        &self,
        name: &str,
    ) -> FluentRequest<'_, request::ComAtprotoServerCreateAppPasswordRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoServerCreateAppPasswordRequest {
                name: name.to_owned(),
            },
        }
    }
    ///Create an invite code.
    pub fn com_atproto_server_create_invite_code(
        &self,
        use_count: i64,
    ) -> FluentRequest<'_, request::ComAtprotoServerCreateInviteCodeRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoServerCreateInviteCodeRequest {
                for_account: None,
                use_count,
            },
        }
    }
    ///Create invite codes.
    pub fn com_atproto_server_create_invite_codes(
        &self,
        code_count: i64,
        use_count: i64,
    ) -> FluentRequest<'_, request::ComAtprotoServerCreateInviteCodesRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoServerCreateInviteCodesRequest {
                code_count,
                for_accounts: None,
                use_count,
            },
        }
    }
    ///Create an authentication session.
    pub fn com_atproto_server_create_session(
        &self,
        identifier: &str,
        password: &str,
    ) -> FluentRequest<'_, request::ComAtprotoServerCreateSessionRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoServerCreateSessionRequest {
                identifier: identifier.to_owned(),
                password: password.to_owned(),
            },
        }
    }
    ///Deactivates a currently active account. Stops serving of repo, and future writes to repo until reactivated. Used to finalize account migration with the old host after the account has been activated on the new host.
    pub fn com_atproto_server_deactivate_account(
        &self,
    ) -> FluentRequest<'_, request::ComAtprotoServerDeactivateAccountRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoServerDeactivateAccountRequest {
                delete_after: None,
            },
        }
    }
    ///Delete an actor's account with a token and password. Can only be called after requesting a deletion token. Requires auth.
    pub fn com_atproto_server_delete_account(
        &self,
        did: &str,
        password: &str,
        token: &str,
    ) -> FluentRequest<'_, request::ComAtprotoServerDeleteAccountRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoServerDeleteAccountRequest {
                did: did.to_owned(),
                password: password.to_owned(),
                token: token.to_owned(),
            },
        }
    }
    ///Delete the current session. Requires auth.
    pub fn com_atproto_server_delete_session(
        &self,
    ) -> FluentRequest<'_, request::ComAtprotoServerDeleteSessionRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoServerDeleteSessionRequest {
            },
        }
    }
    ///Describes the server's account creation requirements and capabilities. Implemented by PDS.
    pub fn com_atproto_server_describe_server(
        &self,
    ) -> FluentRequest<'_, request::ComAtprotoServerDescribeServerRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoServerDescribeServerRequest {
            },
        }
    }
    ///Get all invite codes for the current account. Requires auth.
    pub fn com_atproto_server_get_account_invite_codes(
        &self,
    ) -> FluentRequest<'_, request::ComAtprotoServerGetAccountInviteCodesRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoServerGetAccountInviteCodesRequest {
                create_available: None,
                include_used: None,
            },
        }
    }
    ///Get a signed token on behalf of the requesting DID for the requested service.
    pub fn com_atproto_server_get_service_auth(
        &self,
        aud: &str,
    ) -> FluentRequest<'_, request::ComAtprotoServerGetServiceAuthRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoServerGetServiceAuthRequest {
                aud: aud.to_owned(),
            },
        }
    }
    ///Get information about the current auth session. Requires auth.
    pub fn com_atproto_server_get_session(
        &self,
    ) -> FluentRequest<'_, request::ComAtprotoServerGetSessionRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoServerGetSessionRequest {
            },
        }
    }
    ///List all App Passwords.
    pub fn com_atproto_server_list_app_passwords(
        &self,
    ) -> FluentRequest<'_, request::ComAtprotoServerListAppPasswordsRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoServerListAppPasswordsRequest {
            },
        }
    }
    ///Refresh an authentication session. Requires auth using the 'refreshJwt' (not the 'accessJwt').
    pub fn com_atproto_server_refresh_session(
        &self,
    ) -> FluentRequest<'_, request::ComAtprotoServerRefreshSessionRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoServerRefreshSessionRequest {
            },
        }
    }
    ///Initiate a user account deletion via email.
    pub fn com_atproto_server_request_account_delete(
        &self,
    ) -> FluentRequest<'_, request::ComAtprotoServerRequestAccountDeleteRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoServerRequestAccountDeleteRequest {
            },
        }
    }
    ///Request an email with a code to confirm ownership of email.
    pub fn com_atproto_server_request_email_confirmation(
        &self,
    ) -> FluentRequest<'_, request::ComAtprotoServerRequestEmailConfirmationRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoServerRequestEmailConfirmationRequest {
            },
        }
    }
    ///Request a token in order to update email.
    pub fn com_atproto_server_request_email_update(
        &self,
    ) -> FluentRequest<'_, request::ComAtprotoServerRequestEmailUpdateRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoServerRequestEmailUpdateRequest {
            },
        }
    }
    ///Initiate a user account password reset via email.
    pub fn com_atproto_server_request_password_reset(
        &self,
        email: &str,
    ) -> FluentRequest<'_, request::ComAtprotoServerRequestPasswordResetRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoServerRequestPasswordResetRequest {
                email: email.to_owned(),
            },
        }
    }
    ///Reserve a repo signing key, for use with account creation. Necessary so that a DID PLC update operation can be constructed during an account migraiton. Public and does not require auth; implemented by PDS. NOTE: this endpoint may change when full account migration is implemented.
    pub fn com_atproto_server_reserve_signing_key(
        &self,
    ) -> FluentRequest<'_, request::ComAtprotoServerReserveSigningKeyRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoServerReserveSigningKeyRequest {
                did: None,
            },
        }
    }
    ///Reset a user account password using a token.
    pub fn com_atproto_server_reset_password(
        &self,
        password: &str,
        token: &str,
    ) -> FluentRequest<'_, request::ComAtprotoServerResetPasswordRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoServerResetPasswordRequest {
                password: password.to_owned(),
                token: token.to_owned(),
            },
        }
    }
    ///Revoke an App Password by name.
    pub fn com_atproto_server_revoke_app_password(
        &self,
        name: &str,
    ) -> FluentRequest<'_, request::ComAtprotoServerRevokeAppPasswordRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoServerRevokeAppPasswordRequest {
                name: name.to_owned(),
            },
        }
    }
    ///Update an account's email.
    pub fn com_atproto_server_update_email(
        &self,
        email: &str,
    ) -> FluentRequest<'_, request::ComAtprotoServerUpdateEmailRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoServerUpdateEmailRequest {
                email: email.to_owned(),
                token: None,
            },
        }
    }
    ///Get a blob associated with a given account. Returns the full blob as originally uploaded. Does not require auth; implemented by PDS.
    pub fn com_atproto_sync_get_blob(
        &self,
        cid: &str,
        did: &str,
    ) -> FluentRequest<'_, request::ComAtprotoSyncGetBlobRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoSyncGetBlobRequest {
                cid: cid.to_owned(),
                did: did.to_owned(),
            },
        }
    }
    ///Get data blocks from a given repo, by CID. For example, intermediate MST nodes, or records. Does not require auth; implemented by PDS.
    pub fn com_atproto_sync_get_blocks(
        &self,
        cids: &[&str],
        did: &str,
    ) -> FluentRequest<'_, request::ComAtprotoSyncGetBlocksRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoSyncGetBlocksRequest {
                cids: cids.iter().map(|&x| x.to_owned()).collect(),
                did: did.to_owned(),
            },
        }
    }
    ///DEPRECATED - please use com.atproto.sync.getRepo instead
    pub fn com_atproto_sync_get_checkout(
        &self,
        did: &str,
    ) -> FluentRequest<'_, request::ComAtprotoSyncGetCheckoutRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoSyncGetCheckoutRequest {
                did: did.to_owned(),
            },
        }
    }
    ///Gets the path of repo commits
    pub fn com_atproto_sync_get_commit_path(
        &self,
        did: &str,
    ) -> FluentRequest<'_, request::ComAtprotoSyncGetCommitPathRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoSyncGetCommitPathRequest {
                did: did.to_owned(),
                earliest: None,
                latest: None,
            },
        }
    }
    ///DEPRECATED - please use com.atproto.sync.getLatestCommit instead
    pub fn com_atproto_sync_get_head(
        &self,
        did: &str,
    ) -> FluentRequest<'_, request::ComAtprotoSyncGetHeadRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoSyncGetHeadRequest {
                did: did.to_owned(),
            },
        }
    }
    ///Get the current commit CID & revision of the specified repo. Does not require auth.
    pub fn com_atproto_sync_get_latest_commit(
        &self,
        did: &str,
    ) -> FluentRequest<'_, request::ComAtprotoSyncGetLatestCommitRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoSyncGetLatestCommitRequest {
                did: did.to_owned(),
            },
        }
    }
    ///Get data blocks needed to prove the existence or non-existence of record in the current version of repo. Does not require auth.
    pub fn com_atproto_sync_get_record(
        &self,
        collection: &str,
        did: &str,
        rkey: &str,
    ) -> FluentRequest<'_, request::ComAtprotoSyncGetRecordRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoSyncGetRecordRequest {
                collection: collection.to_owned(),
                commit: None,
                did: did.to_owned(),
                rkey: rkey.to_owned(),
            },
        }
    }
    ///Download a repository export as CAR file. Optionally only a 'diff' since a previous revision. Does not require auth; implemented by PDS.
    pub fn com_atproto_sync_get_repo(
        &self,
        did: &str,
    ) -> FluentRequest<'_, request::ComAtprotoSyncGetRepoRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoSyncGetRepoRequest {
                did: did.to_owned(),
                since: None,
            },
        }
    }
    ///List blob CIDso for an account, since some repo revision. Does not require auth; implemented by PDS.
    pub fn com_atproto_sync_list_blobs(
        &self,
        did: &str,
    ) -> FluentRequest<'_, request::ComAtprotoSyncListBlobsRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoSyncListBlobsRequest {
                cursor: None,
                did: did.to_owned(),
                limit: None,
                since: None,
            },
        }
    }
    ///Enumerates all the DID, rev, and commit CID for all repos hosted by this service. Does not require auth; implemented by PDS and Relay.
    pub fn com_atproto_sync_list_repos(
        &self,
    ) -> FluentRequest<'_, request::ComAtprotoSyncListReposRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoSyncListReposRequest {
                cursor: None,
                limit: None,
            },
        }
    }
    ///Check accounts location in signup queue.
    pub fn com_atproto_temp_check_signup_queue(
        &self,
    ) -> FluentRequest<'_, request::ComAtprotoTempCheckSignupQueueRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoTempCheckSignupQueueRequest {
            },
        }
    }
    ///Request a verification code to be sent to the supplied phone number
    pub fn com_atproto_temp_request_phone_verification(
        &self,
        phone_number: &str,
    ) -> FluentRequest<'_, request::ComAtprotoTempRequestPhoneVerificationRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoTempRequestPhoneVerificationRequest {
                phone_number: phone_number.to_owned(),
            },
        }
    }
    ///Upgrade a repo to v3
    pub fn com_atproto_temp_upgrade_repo_version(
        &self,
        did: &str,
    ) -> FluentRequest<'_, request::ComAtprotoTempUpgradeRepoVersionRequest> {
        FluentRequest {
            client: self,
            params: request::ComAtprotoTempUpgradeRepoVersionRequest {
                did: did.to_owned(),
                force: None,
            },
        }
    }
    ///Administrative action to create a new, re-usable communication (email for now) template.
    pub fn tools_ozone_communication_create_template(
        &self,
        content_markdown: &str,
        name: &str,
        subject: &str,
    ) -> FluentRequest<'_, request::ToolsOzoneCommunicationCreateTemplateRequest> {
        FluentRequest {
            client: self,
            params: request::ToolsOzoneCommunicationCreateTemplateRequest {
                content_markdown: content_markdown.to_owned(),
                created_by: None,
                name: name.to_owned(),
                subject: subject.to_owned(),
            },
        }
    }
    ///Delete a communication template.
    pub fn tools_ozone_communication_delete_template(
        &self,
        id: &str,
    ) -> FluentRequest<'_, request::ToolsOzoneCommunicationDeleteTemplateRequest> {
        FluentRequest {
            client: self,
            params: request::ToolsOzoneCommunicationDeleteTemplateRequest {
                id: id.to_owned(),
            },
        }
    }
    ///Get list of all communication templates.
    pub fn tools_ozone_communication_list_templates(
        &self,
    ) -> FluentRequest<'_, request::ToolsOzoneCommunicationListTemplatesRequest> {
        FluentRequest {
            client: self,
            params: request::ToolsOzoneCommunicationListTemplatesRequest {
            },
        }
    }
    ///Administrative action to update an existing communication template. Allows passing partial fields to patch specific fields only.
    pub fn tools_ozone_communication_update_template(
        &self,
        id: &str,
    ) -> FluentRequest<'_, request::ToolsOzoneCommunicationUpdateTemplateRequest> {
        FluentRequest {
            client: self,
            params: request::ToolsOzoneCommunicationUpdateTemplateRequest {
                content_markdown: None,
                disabled: None,
                id: id.to_owned(),
                name: None,
                subject: None,
                updated_by: None,
            },
        }
    }
    ///Take a moderation action on an actor.
    pub fn tools_ozone_moderation_emit_event(
        &self,
        created_by: &str,
        event: serde_json::Value,
        subject: serde_json::Value,
    ) -> FluentRequest<'_, request::ToolsOzoneModerationEmitEventRequest> {
        FluentRequest {
            client: self,
            params: request::ToolsOzoneModerationEmitEventRequest {
                created_by: created_by.to_owned(),
                event,
                subject,
                subject_blob_cids: None,
            },
        }
    }
    ///Get details about a moderation event.
    pub fn tools_ozone_moderation_get_event(
        &self,
        id: i64,
    ) -> FluentRequest<'_, request::ToolsOzoneModerationGetEventRequest> {
        FluentRequest {
            client: self,
            params: request::ToolsOzoneModerationGetEventRequest {
                id,
            },
        }
    }
    ///Get details about a record.
    pub fn tools_ozone_moderation_get_record(
        &self,
        uri: &str,
    ) -> FluentRequest<'_, request::ToolsOzoneModerationGetRecordRequest> {
        FluentRequest {
            client: self,
            params: request::ToolsOzoneModerationGetRecordRequest {
                cid: None,
                uri: uri.to_owned(),
            },
        }
    }
    ///Get details about a repository.
    pub fn tools_ozone_moderation_get_repo(
        &self,
        did: &str,
    ) -> FluentRequest<'_, request::ToolsOzoneModerationGetRepoRequest> {
        FluentRequest {
            client: self,
            params: request::ToolsOzoneModerationGetRepoRequest {
                did: did.to_owned(),
            },
        }
    }
    ///List moderation events related to a subject.
    pub fn tools_ozone_moderation_query_events(
        &self,
    ) -> FluentRequest<'_, request::ToolsOzoneModerationQueryEventsRequest> {
        FluentRequest {
            client: self,
            params: request::ToolsOzoneModerationQueryEventsRequest {
                added_labels: None,
                added_tags: None,
                comment: None,
                created_after: None,
                created_before: None,
                created_by: None,
                cursor: None,
                has_comment: None,
                include_all_user_records: None,
                limit: None,
                removed_labels: None,
                removed_tags: None,
                report_types: None,
                sort_direction: None,
                subject: None,
                types: None,
            },
        }
    }
    ///View moderation statuses of subjects (record or repo).
    pub fn tools_ozone_moderation_query_statuses(
        &self,
    ) -> FluentRequest<'_, request::ToolsOzoneModerationQueryStatusesRequest> {
        FluentRequest {
            client: self,
            params: request::ToolsOzoneModerationQueryStatusesRequest {
                appealed: None,
                comment: None,
                cursor: None,
                exclude_tags: None,
                ignore_subjects: None,
                include_muted: None,
                last_reviewed_by: None,
                limit: None,
                reported_after: None,
                reported_before: None,
                review_state: None,
                reviewed_after: None,
                reviewed_before: None,
                sort_direction: None,
                sort_field: None,
                subject: None,
                tags: None,
                takendown: None,
            },
        }
    }
    ///Find repositories based on a search term.
    pub fn tools_ozone_moderation_search_repos(
        &self,
    ) -> FluentRequest<'_, request::ToolsOzoneModerationSearchReposRequest> {
        FluentRequest {
            client: self,
            params: request::ToolsOzoneModerationSearchReposRequest {
                cursor: None,
                limit: None,
                q: None,
                term: None,
            },
        }
    }
}
