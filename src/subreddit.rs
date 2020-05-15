use crate::reddit::Reddit;
use crate::models::{
    SubredditInfo,
};
use crate::ChildRedditItem;
use crate::post::Post;
use crate::endpoints::{self, SearchSort};

use crate::search::SubredditSearch;

use std::io;

pub struct SubredditLink<'r> {
    pub reddit: &'r Reddit,
    pub subreddit: String,
}

impl<'r> SubredditLink<'r> {
    pub fn new(reddit: &'r Reddit, subreddit: &str) -> SubredditLink<'r>{
        SubredditLink{
            reddit: reddit,
            subreddit: subreddit.to_owned()
        }
    }

    pub async fn get(self) -> io::Result<Subreddit<'r>> {
        let ep = endpoints::SUBREDDIT_ABOUT.subreddit(&self.subreddit);
        let info = self.reddit.get_data::<SubredditInfo>(ep).await?;

        Ok(Subreddit {
            link: self,
            info: info.data
        })
    }

    pub async fn top(&self) -> io::Result<Vec<Post<'r>>> {
        let ep =  endpoints::SUBREDDIT_TOP.subreddit(&self.subreddit);
        Ok(Post::list_of(self.reddit, &self.reddit.get_list(ep).await?))
    }

    pub async fn search<'s>(&'r self, query: &'s str, sort: SearchSort) -> io::Result<SubredditSearch<'r, 's>> {
        let search_ep = endpoints::SUBREDDIT_SEARCH.subreddit(&self.subreddit);
        let res : SubredditSearch = SubredditSearch::new_search(self.reddit, search_ep, query, sort).await?;
        Ok(res)
    }

}

pub struct Subreddit<'r> {
    pub link: SubredditLink<'r>,
    pub info: SubredditInfo,
}

impl<'r> Subreddit<'r> {
    pub fn info(&self) -> &SubredditInfo {
        &self.info
    }

    pub fn name(&self) -> &str{
        &self.link.subreddit
    }

    pub async fn top(&self) -> io::Result<Vec<Post<'r>>> {
       self.link.top().await
    }

    pub async fn search<'s>(&'r self, query: &'s str, sort: SearchSort) -> io::Result<SubredditSearch<'r, 's>> {
        self.link.search(query, sort).await
    }
}

//Temp
impl<'r> ChildRedditItem<'r> for Subreddit<'r> {
    type DataType = Subreddit<'r>;
    type Metadata = SubredditInfo;

    fn from_parent(reddit: &'r Reddit, info: Self::Metadata) -> Subreddit<'r> {
        Subreddit {
            link:SubredditLink::new(reddit, &info.display_name),
            info:info
        }
    }
}