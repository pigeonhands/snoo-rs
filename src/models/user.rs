use serde::Deserialize;

use crate::models::SubredditInfo;

#[derive(Deserialize, Debug, Clone)]
pub struct UserInfo {
    #[serde(rename = "is_gold")]
    pub has_gold: bool,

    #[serde(rename = "verified")]
    pub is_verified: bool,

    pub name: String,
    pub has_subscribed: bool,
    pub has_verified_email: Option<bool>,
    pub is_employee: bool,
    pub is_friend: bool,
    pub is_mod: bool,
    pub link_karma: i32,
    pub comment_karma: i32,
    pub hide_from_robots: bool,
    pub pref_show_snoovatar: bool,
    pub icon_img: Option<String>,
    pub subreddit: Option<SubredditInfo>,
}
