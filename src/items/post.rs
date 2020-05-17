use crate::items::{
    submission::Submission, subreddit::SubredditLink, user::RedditUserLink, AbstractedApi,
};
use crate::models::{
    SubredditSubmitResponse,
    PostInfo,
    PostEditText,
    PostSetFlair,
    SendComment
};
use crate::reddit::Reddit;
use crate::endpoints;
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

    pub fn name(&self) -> &str {
        self.info.moderate_data.name.as_ref()
    }

    
    pub fn title(&self) -> &str {
        self.info.title.as_ref()
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

    pub async fn comment(&self, message: &str) -> io::Result<SubredditSubmitResponse> {
        let target_url = self.reddit.ep(endpoints::COMMENT)?;
        self.reddit.post_data::<_, SubredditSubmitResponse>(target_url, &SendComment{
            thing_id: self.name(),
            text: message,
        }).await
    }

    pub async fn set_flair(&self, flair_text: &str, flair_class: &str) -> io::Result<()> {
        let target_url = self.reddit.ep(endpoints::FLAIR.subreddit(&self.info.subreddit))?;
        self.reddit.post_data(target_url, &PostSetFlair {
            link: self.name(),
            text: flair_text,
            css_class: flair_class
        }).await
    }

    pub async fn edit_text(&self, new_text: &str) -> io::Result<()> {
        let target_url = self.reddit.ep(endpoints::EDIT)?;
        self.reddit.post_data(target_url, &PostEditText {
            thing_id: self.name(),
            new_text
        }).await
    }

    pub async fn set_sticky(&self, stickied: bool) -> io::Result<()> {
        let target_url = self.reddit.ep(endpoints::STICKY_SUBMISSION)?;
        self.reddit.set_state(target_url, self.name(), stickied).await
    }
}
