use crate::endpoints::{Endpoint, SearchSort};
use crate::models::{ListingData, RedditResponse, RedditResponseGeneric};

use crate::search::{PostSearch, SubredditSearch};
use crate::{endpoints, submission::Submission, subreddit::SubredditLink, user::RedditUserLink};

use reqwest::{Client, Url};
use serde::de::DeserializeOwned;
use std::io;

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

/// Reddit client instance
#[derive(Clone)]
pub struct Reddit {
    client: Client,
}

impl Reddit {
    pub fn new() -> io::Result<Reddit> {
        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Interrupted,
                    format!("Failed to create http client. {:?}", e),
                )
            })?;

        Ok(Reddit { client: client })
    }

    pub(crate) async fn create_request<T: DeserializeOwned>(
        &self,
        target_url: Url,
    ) -> io::Result<T> {
        let resp = self.client.get(target_url).send().await.map_err(|e| {
            io::Error::new(
                io::ErrorKind::ConnectionAborted,
                format!("Failed to send get request. {}", e),
            )
        })?;
        if !resp.status().is_success() {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("A non-success http response was retuned: {}", resp.status()),
            ))?;
        }
        let data = resp.json::<T>().await.map_err(|e| {
            io::Error::new(
                io::ErrorKind::ConnectionAborted,
                format!("Failed to deseralize response. {}", e),
            )
        })?;
        Ok(data)
    }

    pub(crate) async fn get_data<T: DeserializeOwned>(
        &self,
        ep: Endpoint,
    ) -> io::Result<RedditResponseGeneric<T>> {
        self.create_request::<RedditResponseGeneric<T>>(ep.as_api_endpoint()?)
            .await
    }

    pub(crate) async fn get_list<'r, T: DeserializeOwned>(
        &'r self,
        ep: Endpoint,
    ) -> io::Result<Vec<T>> {
        let data = self.get_data::<ListingData<T>>(ep).await?;
        let infos = data.data.inner_children();
        Ok(infos)
    }

    // Get a user by name
    pub fn user<'r>(&'r self, username: &str) -> RedditUserLink<'r> {
        RedditUserLink::new(self, username)
    }

    //get a subreddit by name
    pub fn subreddit<'r>(&'r self, name: &str) -> SubredditLink<'r> {
        SubredditLink::new(self, name)
    }

    /// Search over all of reddit
    pub async fn search<'r, 's>(
        &'r self,
        query: &'s str,
        sort: SearchSort,
    ) -> io::Result<PostSearch<'r, 's>> {
        let search_ep = endpoints::SEARCH;
        PostSearch::new_search(self, search_ep, query, sort).await
    }

    /// Search for a subreddit
    pub async fn search_subreddits<'r, 's>(
        &'r self,
        query: &'s str,
        sort: SearchSort,
    ) -> io::Result<SubredditSearch<'r, 's>> {
        let search_ep = endpoints::SUBREDDITS_SEARCH;
        SubredditSearch::new_search(self, search_ep, query, sort).await
    }

    /// Get post info
    /// a "Submission" is a post + comments
    pub async fn submission_from_link<'a>(&'a self, url: &str) -> io::Result<Submission<'a>> {
        let page_link = Endpoint::new(url);
        let post_data = self
            .get_data::<ListingData<RedditResponse>>(page_link)
            .await?;
        Ok(Submission::from_resp(self, post_data.data))
    }
}
