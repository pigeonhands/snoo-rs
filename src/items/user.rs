use crate::models::UserInfo;
use crate::{endpoints, reddit::Reddit};

use crate::items::{post::Post, submission::Comment, AbstractedApi};

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
        let ep = self
            .reddit
            .ep(endpoints::USER_SUBMITTED.user(&self.username))?;
        Ok(Post::list_of(self.reddit, &self.reddit.get_list(ep).await?))
    }

    pub async fn comments(&self) -> io::Result<Vec<Comment<'r>>> {
        let ep = self
            .reddit
            .ep(endpoints::USER_COMMENTS.user(&self.username))?;
        Ok(Comment::list_of(
            self.reddit,
            &self.reddit.get_list(ep).await?,
        ))
    }

    pub async fn get(self) -> io::Result<RedditUser<'r>> {
        let ep = self.reddit.ep(endpoints::USER_ABOUT.user(&self.username))?;
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

impl RedditUser<'_> {
    pub fn name(&self) -> &str {
        self.info.name.as_ref()
    }

    pub fn info(&self) -> &UserInfo {
        &self.info
    }

    pub fn is_moderator(&self) -> bool {
        self.info.is_mod
    }

    pub fn is_verified(&self) -> bool {
        self.info.verified
    }

    pub fn is_employee(&self) -> bool {
        self.info.is_employee
    }

    pub fn has_gold(&self) -> bool {
        self.info.is_gold
    }

    pub async fn submitted(&'_ self) -> io::Result<Vec<Post<'_>>> {
        self.link.submitted().await
    }

    pub async fn comments(&'_ self) -> io::Result<Vec<Comment<'_>>> {
        self.link.comments().await
    }
}

impl<'r> AbstractedApi<'r> for RedditUser<'r> {
    type ApiType = UserInfo;
    type AbstractedType = RedditUser<'r>;

    fn from_parent(reddit: &'r Reddit, info: Self::ApiType) -> RedditUser<'r> {
        RedditUser {
            link: RedditUserLink::new(reddit, &info.name),
            info,
        }
    }
}
