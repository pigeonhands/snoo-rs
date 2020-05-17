use crate::models::{ListingData, RedditResponse, RedditResponseGeneric};

use crate::items::{
    search::{PostSearch, SubredditSearch, UserSearch},
    submission::Submission,
    subreddit::SubredditLink,
    user::RedditUserLink,
    AbstractedApi,
};

use crate::endpoints::{self, Endpoint, EndpointBuilder, SearchSort};

use crate::rate_limit::RateLimiter;
use crate::reddit_app::RedditApp;

use serde::de::DeserializeOwned;
use std::io;

/// An unauthenicated application:
/// ```
/// let r = Reddit::new()?;
/// ```
///
/// An authenicated script application:
/// ```
/// let r = Reddit::new_script("snoo-rs", "password", "id", "secret").await?;
/// ```
///
#[derive(Clone)]
pub struct Reddit {
    pub app: RedditApp,
}

impl Reddit {
    /// Creates a new Reddit instance with a given Application instance
    pub fn from_app(app: RedditApp) -> io::Result<Reddit> {
        Ok(Reddit { app: app })
    }

    /// Creates a new reddit insance with an unauthenicated
    /// and not rate limited [RedditApp].
    /// Same as 
    /// ```Reddit::from_app(RedditApp::new()?)```
    pub fn new() -> io::Result<Reddit> {
        Reddit::from_app(RedditApp::new()?)
    }

    /// Creates a new reddit insance with an
    /// authenitated script [RedditApp].
    pub async fn new_script(
        username: &str,
        password: &str,
        id: &str,
        secret: &str,
    ) -> io::Result<Reddit> {
        Reddit::from_app(RedditApp::new_script(username, password, id, secret).await?)
    }

    /// Takes a Api model and binds it to the
    /// Reddit instance so api calls can be made.
    pub fn bind<'r, T: AbstractedApi<'r>>(&'r self, api_data: T::ApiType) -> T::AbstractedType {
        T::from_parent(self, api_data)
    }

    /// Builds a new endpoint
    /// calls [RedditApp::create_endpoint]
    pub fn ep(&self, builder: EndpointBuilder) -> io::Result<Endpoint> {
        self.app.create_endpoint(builder)
    }

    /// Builds a new endpoint from a string
    /// calls [RedditApp::create_endpoint_str]
    pub fn ep_str(&self, str_ep: &str) -> io::Result<Endpoint> {
        self.app.create_endpoint_str(str_ep)
    }

    /// Creates a rewuest to the reddit api and
    // returns the json `"data"` section as [T]
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

    /// Set the rate limiter for the application
    /// e.g. 
    /// [RateLimiter::new_batched()] or [RateLimiter::new_paced()]
    pub fn rate_limiter(mut self, limiter: RateLimiter) -> Self {
        self.app.rate_limiter = limiter;
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

    /// Get [Submission] from a post url
    pub async fn submission_from_link(&self, url: &'_ str) -> io::Result<Submission<'_>> {
        let page_link = self.ep_str(url)?;
        let post_data = self
            .get_data::<ListingData<RedditResponse>>(page_link)
            .await?;
        Ok(Submission::from_resp(self, post_data.data))
    }
}
