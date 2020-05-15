use crate::models::{ModerateData, RedditResponse, VoteData};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum CommentReplies {
    NoReply(String),
    HasReplies(Vec<RedditResponse>),
}

impl Default for CommentReplies {
    fn default() -> Self {
        CommentReplies::NoReply(String::new())
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct CommentData {
    #[serde(flatten)]
    pub vote_data: VoteData,

    #[serde(flatten)]
    pub moderate_data: ModerateData,

    pub author: String,
    pub body: String,
    pub body_html: String,
    pub parent_id: String,
    pub subreddit: String,
    pub link_id: String,
    pub link_title: String,

    pub replies: CommentReplies,

    pub total_awards_received: i32,
    pub approved_at_utc: Option<f32>,
}
