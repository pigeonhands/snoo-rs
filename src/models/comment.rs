use crate::models::{
    VoteData,
    RedditResponse
};
use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone)]
pub struct CommentData {
    #[serde(flatten)]
    pub vote_data: VoteData,

    pub body: String,
    pub body_html: String,
    pub parent_id: String,
    pub subreddit: String,
    pub link_id: String,
    pub link_title: String,

    pub replies: Vec<RedditResponse>,

    pub total_awards_received: i32,
    pub approved_at_utc: f32,
}