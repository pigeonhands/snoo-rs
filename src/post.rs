use crate::models::PostInfo;
use crate::reddit::Reddit;
use crate::submission::Submission;
use crate::subreddit::SubredditLink;
use crate::user::RedditUserLink;
use crate::ChildRedditItem;

use std::io;

pub struct Post<'r> {
    reddit: &'r Reddit,
    info: PostInfo,
}

impl<'r> ChildRedditItem<'r> for Post<'r> {
    type DataType = Post<'r>;
    type Metadata = PostInfo;

    fn from_parent(parent: &'r Reddit, info: Self::Metadata) -> Self {
        Self {
            reddit: parent,
            info: info,
        }
    }
}

impl<'r> Post<'r> {
    pub fn info(&self) -> &PostInfo {
        &self.info
    }

    pub fn url(&self) -> &str {
        self.info.url.as_ref()
    }

    pub fn subreddit(&self) -> SubredditLink<'r> {
        self.reddit.subreddit(&self.info.subreddit)
    }

    pub fn author(&self) -> RedditUserLink<'r> {
        RedditUserLink::new(self.reddit, &self.info.author)
    }

    pub async fn submission(&self) -> io::Result<Submission<'r>> {
        self.reddit.submission_from_link(&self.url()).await
    }
}
