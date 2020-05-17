use crate::models::{ModerateData, VoteData};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Default)]
pub struct PostImage {
    pub url: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct PostImages {
    pub source: PostImage,
    pub resolutions: Vec<PostImage>,
    pub id: String,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct PostPreview {
    pub images: Vec<PostImages>,
    pub enabled: bool,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct PostInfo {
    #[serde(flatten)]
    pub vote_data: VoteData,

    #[serde(flatten)]
    pub moderate_data: ModerateData,

    #[serde(rename = "spoiler")]
    pub is_spoiler: bool,

    #[serde(rename = "hidden")]
    pub is_hidden: bool,

    #[serde(rename = "is_self")]
    pub is_self_posted: bool,

    #[serde(rename = "over_18")]
    pub nsfw: bool,

    pub author: String,
    pub permalink: Option<String>,
    pub domain: Option<String>,
    pub link_flair_css_class: Option<String>,
    pub link_flair_text: Option<String>,
    pub num_comments: i32,
    pub selftext: Option<String>,
    pub selftext_html: Option<String>,
    pub thumbnail: Option<String>,
    pub preview: Option<PostPreview>,
    pub title: String,
    pub subreddit: String,
    pub url: String,
    pub is_crosspostable: bool,
    pub num_crossposts: i32,
    pub created: f64,
    pub crosspost_parent: Option<String>,
}

use crate::feed::Feedable;
impl Feedable for PostInfo {
    fn feed_id(&self) -> String {
        self.moderate_data.name.clone()
    }
}

#[derive(Serialize)]
pub struct PostSetFlair<'a> {
    pub css_class: &'a str,
    pub link: &'a str,
    pub text: &'a str,
}


#[derive(Serialize)]
pub struct PostEditText<'a> {
    pub thing_id: &'a str,
    pub new_text: &'a str,
}
