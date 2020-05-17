use crate::items::{
    submission::Submission, subreddit::SubredditLink, user::RedditUserLink, AbstractedApi,
};
use crate::models::PostInfo;
use crate::reddit::Reddit;

use std::io;

pub struct Post<'r> {
    reddit: &'r Reddit,
    info: PostInfo,
}

impl<'r> AbstractedApi<'r> for Post<'r> {
    type ApiType = PostInfo;
    type AbstractedType = Post<'r>;

    fn from_parent(parent: &'r Reddit, info: Self::ApiType) -> Self {
        Self {
            reddit: parent,
            info: info,
        }
    }
}

impl Post<'_> {
    /// Returns the underlying [PostInfo] model.
    pub fn info(&self) -> &PostInfo {
        &self.info
    }

    pub fn url(&self) -> &str {
        self.info.url.as_ref()
    }

    pub fn subreddit(&'_ self) -> SubredditLink {
        self.reddit.subreddit(&self.info.subreddit)
    }

    pub fn author(&self) -> RedditUserLink {
        RedditUserLink::new(self.reddit, &self.info.author)
    }

    pub async fn submission(&'_ self) -> io::Result<Submission<'_>> {
        self.reddit.submission_from_link(&self.url()).await
    }
}
