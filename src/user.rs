use crate::models::UserInfo;
use crate::{endpoints, post::Post, reddit::Reddit, submission::Comment};

use crate::ChildRedditItem;
use std::io;

/// Weak link to the user.
// Dosent perform any http request when created.
// calling .get will fetch the data
pub struct RedditUserLink<'r> {
    reddit: &'r Reddit,
    username: String,
}

impl<'r> RedditUserLink<'r> {
    pub fn new(reddit: &'r Reddit, name: &str) -> RedditUserLink<'r> {
        RedditUserLink {
            reddit: reddit,
            username: name.to_owned(),
        }
    }

    pub async fn submitted(&self) -> io::Result<Vec<Post<'r>>> {
        let ep = endpoints::USER_SUBMITTED.user(&self.username);
        Ok(Post::list_of(self.reddit, &self.reddit.get_list(ep).await?))
    }

    pub async fn comments(&self) -> io::Result<Vec<Comment<'r>>> {
        let ep = endpoints::USER_COMMENTS.user(&self.username);
        Ok(Comment::list_of(
            self.reddit,
            &self.reddit.get_list(ep).await?,
        ))
    }

    pub async fn get(self) -> io::Result<RedditUser<'r>> {
        let ep = endpoints::USER_ABOUT.user(&self.username);
        let about = self.reddit.get_data::<UserInfo>(ep).await?;

        Ok(RedditUser {
            link: self,
            info: about.data,
        })
    }
}

/// Full user infomation
pub struct RedditUser<'r> {
    link: RedditUserLink<'r>,
    info: UserInfo,
}

impl<'r> RedditUser<'r> {
    pub fn name(&self) -> &str {
        self.info.name.as_ref()
    }

    pub fn info(&self) -> &UserInfo {
        &self.info
    }

    pub async fn submitted(&self) -> io::Result<Vec<Post<'r>>> {
        self.link.submitted().await
    }

    pub async fn comments(&self) -> io::Result<Vec<Comment<'r>>> {
        self.link.comments().await
    }
}

impl<'r> ChildRedditItem<'r> for RedditUser<'r> {
    type DataType = RedditUser<'r>;
    type Metadata = UserInfo;

    fn from_parent(reddit: &'r Reddit, info: Self::Metadata) -> RedditUser<'r> {
        RedditUser {
            link: RedditUserLink::new(reddit, &info.name),
            info: info,
        }
    }
}

pub struct UserOverview {}
