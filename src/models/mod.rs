//! Models for the reddit json api
pub mod auth;
mod comment;
mod listing;
mod metadata;
mod post;
mod search;
mod subreddit;
mod user;

use serde::{Deserialize, Serialize};

pub use crate::models::{
    comment::{CommentData, SendComment},
    listing::ListingData,
    metadata::{ModerateData, VoteData},
    post::{PostImage, PostImages, PostInfo, PostPreview, PostSetFlair, PostEditText},
    search::SearchInfo,
    subreddit::{SubredditInfo, SubredditSubmit, SubredditSubmitLink, SubredditSubmitText, SubredditSubmitResponse},
    user::UserInfo,
};

#[derive(Deserialize)]
pub struct EmptyResponse();


#[derive(Deserialize, Clone, Debug)]
pub struct RedditPostResponseJson<T> {
    pub errors: Vec<String>,
    pub data: Option<T>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RedditPostResponse<T> {
    pub json: RedditPostResponseJson<T>,
}




#[derive(Serialize, Clone, Debug)]
pub struct RedditJsonApiType<T> {
    pub api_type: &'static str,
    #[serde(flatten)]
    pub data: T,
}

impl<T> RedditJsonApiType<T>{
    pub fn new(data: T) -> Self{
        Self{
            api_type: "json",
            data
        }
    }
}


#[derive(Serialize, Clone, Debug)]
pub struct RedditSetState<'a, T> {
    pub id: &'a str,
    pub state: T,
}


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
