use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct SubredditInfo {
    pub created: Option<f64>,
    pub description: String,
    pub description_html: Option<String>,
    pub display_name: String,
    pub header_img: Option<String>,
    pub header_title: Option<String>,
    pub public_description: String,
    pub subscribers: Option<i32>,
    pub accounts_active: Option<i32>,
    pub title: String,
    pub url: String,
    pub user_is_moderator: Option<bool>,
    pub mod_permissions: Option<i32>,
    pub user_is_banned: Option<bool>,
}


#[derive(Serialize)]
pub struct SubredditSubmit<'a> {
    ///  "link", "self" or "image"
    pub kind: &'a str,
    /// Subreddit
    pub sr : &'a str,
    pub title:  &'a str,
    pub resubmit: bool,
    #[serde(skip_serializing_if="Option::is_none")]
    pub iden:  Option<&'a str>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub captcha:  Option<&'a str>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SubredditSubmitResponse {
    pub url: String,
    pub id: String,
    pub name: String,
    pub drafts_count: Option<i32>,
}


#[derive(Serialize)]
pub struct SubredditSubmitLink<'a> {
    #[serde(flatten)]
    pub submit: SubredditSubmit<'a>,
    pub url:  &'a str,
}

#[derive(Serialize)]
pub struct SubredditSubmitText<'a> {
    #[serde(flatten)]
    pub submit: SubredditSubmit<'a>,
    pub text:  &'a str,
}

