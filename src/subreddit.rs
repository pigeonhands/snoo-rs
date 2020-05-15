use crate::reddit::Reddit;
use crate::models::{
    RedditResponse,
    SubredditInfo,
    ListingData,
    PostInfo
};
use crate::ChildRedditItem;
use crate::post::Post;
use crate::endpoints;

use std::io;

pub struct Subreddit<'r> {
    pub (crate) reddit: &'r Reddit,
    pub (crate) name: String,
}

impl<'r> Subreddit<'r> {

    pub async fn info(&self) -> io::Result<SubredditInfo> {
        let res = self.reddit.get_any(endpoints::SUBREDDIT_ABOUT.subreddit(&self.name)).await?;

        if let RedditResponse::Subreddit(info) = res {
            Ok(info)
        }else{
            Err(io::Error::new(io::ErrorKind::InvalidData, "Unexpected model recieved"))
        }
    }

    pub async fn top(&self) -> io::Result<Vec<Post<'r>>> {
        let ep = endpoints::SUBREDDIT_TOP.subreddit(&self.name);
        let posts = self.reddit.get_data::<ListingData<PostInfo>>(ep.as_api_endpoint()?).await?;
        let post_info = posts.data.inner_children();
        Ok(Post::list_of(self.reddit, &post_info))
    }
}