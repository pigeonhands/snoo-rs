use crate::subreddit::Subreddit;
use crate::endpoints::Endpoint;
use crate::models::{
    RedditResponseGeneric,
    RedditResponse,
    ListingData
};

use crate::{
    post::Post,
    submission::Submission,
};

use std::io;
use reqwest::{Client};
use serde::de::{DeserializeOwned};
use serde_json;


static USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

/// Reddit client instance
#[derive(Clone)]
pub struct Reddit{
    client: Client,
}

impl Reddit {

    pub fn new() -> io::Result<Reddit> {
        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .build().map_err(|e| io::Error::new(io::ErrorKind::Interrupted, format!("Failed to create http client. {:?}", e)))?;

        Ok(Reddit{
            client: client
        })
    }
    async fn create_request<T: DeserializeOwned>(&self, ep: Endpoint) -> io::Result<T> {
        let target_url = ep.as_api_endpoint()?;
        let data = self.client.get(target_url)
            .send().await.map_err(|e| io::Error::new(io::ErrorKind::ConnectionAborted, format!("Failed to send get request. {}", e)))?
            .json::<T>()
            .await.map_err(|e| io::Error::new(io::ErrorKind::ConnectionAborted, format!("Failed to deseralize response. {}", e)))?;
        
        //println!("{}", data);
        //Ok(serde_json::from_str::<T>(&data)?)
        Ok(data)
    }

    pub (crate) async fn get_data<T: DeserializeOwned>(&self, ep: Endpoint) -> io::Result<RedditResponseGeneric<T>> {
        self.create_request::<RedditResponseGeneric<T>>(ep).await
    }

    pub (crate) async fn get_any(&self, ep: Endpoint) -> io::Result<RedditResponse> {
        self.create_request::<RedditResponse>(ep).await
    }

    pub async fn submission_from_link<'a>(&'a self, url: &str) -> io::Result<Submission<'a>>{
        let ep = Endpoint::new(url);
        let post_data = self.get_data::<ListingData<RedditResponse>>(ep).await?;
        Ok(Submission::from_resp(self, post_data.data))
    }

    pub fn subreddit(&self, name: &str) -> Subreddit {
        Subreddit{
            reddit: self,
            name: name.to_owned()
        }
    }

}