//! Reddit client.
use crate::models::{
    PostInfo,
    CommentData,
    ListingData, 
    RedditResponseGeneric,
    RedditJsonApiType
};

use crate::items::{
    search::{PostSearch, SubredditSearch, UserSearch},
    submission::Submission,
    subreddit::SubredditLink,
    user::RedditUserLink,
    AbstractedApi,
};

use crate::endpoints::{self, Endpoint, EndpointBuilder, SearchSort};

use crate::rate_limit::RateLimiter;
use crate::reddit_api::RedditApi;

use serde::{de::DeserializeOwned, Serialize};
use std::io;

/// A new reddit client.
/// ```
/// // An unauthenicated application:
/// let r = Reddit::new()?;
/// ```
///
///
/// ```
/// // An authenicated script application
/// let r = Reddit::new_script("snoo-rs", "password", "id", "secret").await?;
/// ```
///
#[derive(Clone)]
pub struct Reddit {
    pub api: RedditApi,
}

impl Reddit {
    /// Creates a new Reddit instance with a given Application instance
    pub fn from_api(api: RedditApi) -> io::Result<Reddit> {
        Ok(Reddit { api })
    }

    /// Creates a new reddit insance with an unauthenicated
    /// and not rate limited [RedditApi].
    /// Same as
    /// ```Reddit::from_app(RedditApi::new()?)```
    pub fn new() -> io::Result<Reddit> {
        Reddit::from_api(RedditApi::new()?)
    }

    /// Creates a new reddit insance with an
    /// authenitated script [RedditApi].
    pub async fn new_script(
        username: &str,
        password: &str,
        id: &str,
        secret: &str,
    ) -> io::Result<Reddit> {
        let mut r = Reddit::new()?;
        r.api
            .authorize_script(username, password, id, secret)
            .await?;
        Ok(r)
    }

    /// Takes an api model and binds it to the
    /// [Reddit] instance so api calls can be made.
    ///
    /// e.g.
    /// Takes [PostInfo](crate::models::PostInfo) and turns it into [Post](crate::items::Post)
    pub fn bind<'r, T: AbstractedApi<'r>>(&'r self, api_data: T::ApiType) -> T::AbstractedType {
        T::from_parent(self, api_data)
    }

    /// Builds a new endpoint
    /// calls [RedditApi::create_endpoint]
    pub fn ep(&self, builder: EndpointBuilder) -> io::Result<Endpoint> {
        self.api.create_endpoint(builder)
    }

    /// Builds a new endpoint from a string
    /// calls [RedditApi::create_endpoint_str]
    pub fn ep_str(&self, str_ep: &str) -> io::Result<Endpoint> {
        self.api.create_endpoint_str(str_ep)
    }

    /// Creates a rewuest to the a api and
    // returns the json `"data"` section as [T]
    pub(crate) async fn get_data<T: DeserializeOwned>(
        &self,
        ep: Endpoint,
    ) -> io::Result<RedditResponseGeneric<T>> {
        self.api
            .get_api::<RedditResponseGeneric<T>>(ep.to_url())
            .await
    }

    /// Creates a post request to a reddit api
    pub async fn post_data<S:Serialize, R: DeserializeOwned>(&self, target_url: Endpoint, data: &S) -> io::Result<R> {
        self.api.post_api(target_url.to_url(), &RedditJsonApiType::new(data)).await
    }

    /// Sets the state of a thing
    pub async fn set_state<T: Serialize>(&self, target_url: Endpoint, id:&str, state: T) -> io::Result<()>{
        self.api.set_state(target_url.to_url(), id, state).await
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
        self.api.rate_limiter = limiter;
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
    /// TODO: FIX THIS
    pub async fn submission_from_link(&self, url: &'_ str) -> io::Result<Submission<'_>> {
        let page_link = self.ep_str(url)?;

        let (post, comment) = self.api
            .get_api::<(RedditResponseGeneric<ListingData<PostInfo>>, RedditResponseGeneric<ListingData<CommentData>>)>(page_link.to_url())
            .await?;

        Ok(Submission::from_resp(self, post.data, comment.data))
    }
}
