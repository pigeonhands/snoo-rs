use crate::endpoints::{self, Endpoint, SearchSort};
use crate::models::{ListingData, RedditResponse, RedditResponseGeneric};

use crate::items::{
    search::{PostSearch, SubredditSearch, UserSearch},
    submission::Submission,
    subreddit::SubredditLink,
    user::RedditUserLink,
    AbstractedApi,
};

use crate::rate_limit::RateLimiter;
use crate::reddit_app::RedditApp;

use serde::de::DeserializeOwned;
use std::io;

/// Reddit client instance
#[derive(Clone)]
pub struct Reddit {
    pub(crate) app: RedditApp,
}

impl Reddit {
    pub fn new() -> io::Result<Reddit> {
        Ok(Reddit {
            app: RedditApp::new()?,
        })
    }

    pub fn from_app(app: RedditApp) -> io::Result<Reddit> {
        Ok(Reddit { app: app })
    }

    /// No rate limiting.
    pub fn rate_limit_off(mut self) -> Self {
        self.app.rate_limiter = RateLimiter::Off;
        self
    }

    /// Make requests as quick as possable until limit is
    /// reached, then wait for reset.
    pub fn rate_limit_bacthed(mut self) -> Self {
        self.app.rate_limiter = RateLimiter::new_batched();
        self
    }

    /// Every request will wait (requests_avalible) / (time_avalible)
    /// so the delays are evenly spaced.
    pub fn rate_limit_paced(mut self) -> Self {
        self.app.rate_limiter = RateLimiter::new_paced();
        self
    }

    pub(crate) async fn get_data<T: DeserializeOwned>(
        &self,
        ep: Endpoint,
    ) -> io::Result<RedditResponseGeneric<T>> {

        self.app
            .create_request::<RedditResponseGeneric<T>>(ep.as_api_endpoint()?)
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

    /// Takes a Api model and binds it to the
    /// Reddit instance so api calls can be made.
    pub fn bind<'r, T: AbstractedApi<'r>>(&'r self, api_data: T::ApiType) -> T::AbstractedType {
        T::from_parent(self, api_data)
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

    /// Search for a subreddit
    pub async fn search_users<'r, 's>(
        &'r self,
        query: &'s str,
        sort: SearchSort,
    ) -> io::Result<UserSearch<'r, 's>> {
        let search_ep = endpoints::USERS_SEARCH;
        UserSearch::new_search(self, search_ep, query, sort).await
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
