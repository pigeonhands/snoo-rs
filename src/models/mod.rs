mod comment;
mod listing;
mod metadata;
mod post;
mod search;
mod subreddit;
mod user;

use serde::Deserialize;

pub use crate::models::{
    comment::CommentData,
    listing::ListingData,
    metadata::{ModerateData, VoteData},
    post::{PostImage, PostImages, PostInfo, PostPreview},
    search::SearchInfo,
    subreddit::SubredditInfo,
    user::UserInfo,
};

#[derive(Deserialize, Clone, Debug)]
pub struct RedditResponseGeneric<T> {
    pub kind: String,
    pub data: T,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "kind", content = "data")]
pub enum RedditResponse {
    #[serde(rename = "t1")]
    Comment(CommentData),

    #[serde(rename = "t2")]
    RedditUser(UserInfo),

    #[serde(rename = "t3")]
    Post(PostInfo),

    #[serde(rename = "t4")]
    PrivateMessage(),

    #[serde(rename = "t5")]
    Subreddit(SubredditInfo),

    #[serde(rename = "Listing")]
    Listing(ListingData<RedditResponse>),

    #[serde(rename = "modaction")]
    ModAction(),

    #[serde(rename = "more")]
    More(),

    #[serde(rename = "LiveUpdate")]
    LiveUpdate(),

    #[serde(rename = "LiveUpdateEvent")]
    LiveUpdateEvent(),

    Invalid,
}
