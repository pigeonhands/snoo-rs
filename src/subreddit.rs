use crate::reddit::Reddit;
use crate::models::{
    RedditResponse,
    SubredditInfo,
};
use crate::ChildRedditItem;
use crate::post::Post;
use crate::endpoints::{self, SearchSort};

use crate::search::SubredditPostSearch;

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
        let ep =  endpoints::SUBREDDIT_TOP.subreddit(&self.name);
        Ok(Post::list_of(self.reddit, &self.reddit.get_list(ep).await?))
    }

    pub async fn search<'s>(&'r self, query: &'s str, sort: SearchSort) -> io::Result<SubredditPostSearch<'r, 's>> {
        let search_ep = endpoints::SUBREDDIT_SEARCH.subreddit(&self.name);
        let res : SubredditPostSearch = SubredditPostSearch::new_search(self.reddit, search_ep, query, sort).await?;
        Ok(res)
    }
}

//Temp
impl<'r> ChildRedditItem<'r> for Subreddit<'r> {
    type DataType = SubredditInfo;
    type Metadata = SubredditInfo;

    fn from_parent(_: &'r Reddit, info: Self::Metadata) -> SubredditInfo{
        info
    }
}