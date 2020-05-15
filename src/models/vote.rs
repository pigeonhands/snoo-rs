use serde::{Deserialize};
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum VoteEdited{
    IsEdited(bool),
    TimeEdited(f32)
}

impl Default for VoteEdited{
    fn default() -> Self {
        VoteEdited::IsEdited(false)
    }
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct VoteData {
    #[serde(rename="stickied")]
    pub is_stickied: bool,

    #[serde(rename="likes")]
    pub liked: Option<bool>, 

    pub author_flair_css_class: Option<String>,
    pub author_flair_text: Option<String>,
    pub downs: i32,
    pub ups: i32,
    pub edited: VoteEdited,
    pub archived: bool,
    pub saved: bool,
    pub locked: bool,
    pub gilded: i32,
}