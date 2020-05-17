use crate::models::{CommentData, ListingData, PostInfo, RedditResponse, SendComment};

use crate::reddit::Reddit;

use crate::items::{post::Post, user::RedditUserLink, AbstractedApi};
use std::io;
use crate::endpoints;

/// A submission is a full reddit post
/// It is a [Post] with a list of [Comment]s
pub struct Submission<'r> {
    op: Post<'r>,
    comments: Vec<Comment<'r>>,
}

impl<'r> Submission<'r> {
    pub(crate) fn from_resp(reddit: &'r Reddit, op: ListingData<PostInfo>, comments: ListingData<CommentData>) -> Self {

        Self {
            op: reddit.bind::<Post>(op.children[0]),
            comments: comments.children,
        }
    }

    pub fn op(&self) -> &Post {
        &self.op
    }

    pub fn comments(&self) -> &[Comment] {
        &self.comments
    }
}

/// A user comment
pub struct Comment<'r> {
    reddit: &'r Reddit,
    data: CommentData,
}

impl Comment<'_> {
    /// Returns the underlying [CommentData] model.
    pub fn info(&self) -> &CommentData {
        &self.data
    }

    pub fn author(&self) -> RedditUserLink {
        self.reddit.user(&self.data.author)
    }

    pub fn name(&self) -> &str {
        self.data.moderate_data.name.as_str()
    }

    pub async fn reply(&self, message: &str) -> io::Result<()> {
        let target_url = self.reddit.ep(endpoints::COMMENT)?;
        self.reddit.post_data(target_url, &SendComment{
            thing_id: self.name(),
            text: message,
        }).await
    }
}

impl<'r> AbstractedApi<'r> for Comment<'r> {
    type AbstractedType = Comment<'r>;
    type ApiType = CommentData;

    fn from_parent(parent: &'r Reddit, info: Self::ApiType) -> Self {
        Self {
            reddit: parent,
            data: info,
        }
    }
}
