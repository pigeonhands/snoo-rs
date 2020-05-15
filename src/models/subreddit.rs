use chrono::{
    DateTime,
    Utc,
    prelude::*,
};

use serde::{Deserialize};

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

impl SubredditInfo {
    pub fn created_utc(&self) -> Option<DateTime<Utc>> {
        if let Some(created) = self.created{
            let naive_datetime = NaiveDateTime::from_timestamp(created as i64, 0);
            Some(DateTime::from_utc(naive_datetime, Utc))
        }else{
            None
        }
        
    }
}
