use crate::models::{
    PostInfo,
};
use crate::endpoints::Endpoint;
use crate::reddit::Reddit;
use crate::subreddit::Subreddit;
use crate::ChildRedditItem;
use crate::user::RedditUser;
use crate::submission::Submission;

use std::io;

pub struct Post<'r>{
    reddit: &'r Reddit,
    info: PostInfo,
}

impl<'r> ChildRedditItem<'r> for Post<'r> {
    type DataType = Post<'r>;
    type Metadata = PostInfo;

    fn from_parent(parent: &'r Reddit, info: Self::Metadata) -> Self{
        Self{
            reddit: parent,
            info: info,
        }
    }
}

impl<'r> Post<'r> {
    pub fn info(&self) -> &PostInfo {
        &self.info
    }

    pub fn subreddit(&self) -> Subreddit {
        self.reddit.subreddit(&self.info.subreddit)
    }

    pub fn author(&self) -> RedditUser{
        self.reddit.user(&self.info.author)
    }

    pub async fn submission(&self) -> io::Result<Submission<'r>> {
        let submission = self.reddit.submission_from_link(&self.info.url).await?;
        Ok(submission)
    }
}
