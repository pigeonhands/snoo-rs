use crate::models::{ListingData, RedditResponse, RedditResponseGeneric};

use crate::items::{
    search::{PostSearch, SubredditSearch, UserSearch},
    submission::Submission,
    subreddit::SubredditLink,
    user::RedditUserLink,
    AbstractedApi,
};

use crate::endpoints::{self, Endpoint, EndpointBase, EndpointBuilder, SearchSort};

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

    /// Takes a Api model and binds it to the
    /// Reddit instance so api calls can be made.
    pub fn bind<'r, T: AbstractedApi<'r>>(&'r self, api_data: T::ApiType) -> T::AbstractedType {
        T::from_parent(self, api_data)
    }

    /// Builds a new ep
    pub fn ep(&self, builder: EndpointBuilder) -> io::Result<Endpoint> {
        self.app.create_endpoint(builder)
    }

    /// Builds a new ep from a string
    pub fn ep_str(&self, str_ep: &str) -> io::Result<Endpoint> {
        self.app.create_enddpoint_str(str_ep)
    }

    pub(crate) async fn get_data<T: DeserializeOwned>(
        &self,
        ep: Endpoint,
    ) -> io::Result<RedditResponseGeneric<T>> {
        self.app
            .create_request::<RedditResponseGeneric<T>>(ep.to_url())
            .await
    }

    pub(crate) async fn get_list<T: DeserializeOwned>(&self, ep: Endpoint) -> io::Result<Vec<T>> {
        let data = self.get_data::<ListingData<T>>(ep).await?;
        let infos = data.data.inner_children();
        Ok(infos)
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

    // Get a user by name
    pub fn user(&self, username: &str) -> RedditUserLink {
        RedditUserLink::new(self, username)
    }

    //get a subreddit by name
    pub fn subreddit(&self, name: &str) -> SubredditLink {
        SubredditLink::new(self, name)
    }

    /// Search over all of reddit
    pub async fn search<'r, 's>(
        &'r self,
        query: &'s str,
        sort: SearchSort,
    ) -> io::Result<PostSearch<'r, 's>> {
        let search_ep = self.ep(endpoints::SEARCH)?;
        PostSearch::new_search(self, search_ep, query, sort).await
    }

    /// Search for a subreddit
    pub async fn search_subreddits<'r, 's>(
        &'r self,
        query: &'s str,
        sort: SearchSort,
    ) -> io::Result<SubredditSearch<'r, 's>> {
        let search_ep = self.ep(endpoints::SUBREDDITS_SEARCH)?;
        SubredditSearch::new_search(self, search_ep, query, sort).await
    }

    /// Search for a subreddit
    pub async fn search_users<'r, 's>(
        &'r self,
        query: &'s str,
        sort: SearchSort,
    ) -> io::Result<UserSearch<'r, 's>> {
        let search_ep = self.ep(endpoints::USERS_SEARCH)?;
        UserSearch::new_search(self, search_ep, query, sort).await
    }

    /// Get post info
    /// a "Submission" is a post + comments
    pub async fn submission_from_link(&self, url: &'_ str) -> io::Result<Submission<'_>> {
        let page_link = self.ep_str(url)?;
        let post_data = self
            .get_data::<ListingData<RedditResponse>>(page_link)
            .await?;
        Ok(Submission::from_resp(self, post_data.data))
    }
}
