//! A feed of content.
//! e.g.
//! a feed of new [crate::item::submission::Post] from a [crate::item::subreddit::Subreddit]
//!
//! ```
//! let r = Reddit::new()?;
//! let all = r.subreddit("all");
//!
//! let feed = all.feed()?.delay(Duration::from_secs(5));
//! let mut rx = feed.start()?;
//!
//! while let Some(d) = rx.recv().await {
//!     println!("{} \t {}", d.created, d.name);
//! }
//!
//! ```
use crate::reddit::Reddit;

use crate::endpoints::{Endpoint, SearchSort};
use crate::models::{RedditResponseGeneric, SearchInfo};

use std::io;

use std::marker::PhantomData;
use tokio::sync::mpsc;
use tokio::time::{delay_for, Duration};

use serde::de::DeserializeOwned;

pub trait Feedable: Clone + Send + Sync + 'static {
    fn feed_id(&self) -> String;
}

pub struct ContentStream<T>
where
    T: Feedable + DeserializeOwned,
{
    phantom: PhantomData<T>,
    reddit: Reddit,
    endpoint: Endpoint,
    delay: Duration,
}

impl<T> ContentStream<T>
where
    T: Feedable + DeserializeOwned,
{
    pub fn new(reddit: Reddit, search_ep: Endpoint) -> ContentStream<T> {
        ContentStream {
            phantom: PhantomData,
            reddit: reddit,
            endpoint: search_ep,
            delay: Duration::from_secs(3),
        }
    }

    /// delay between polling the api
    /// default: 3s
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    async fn read_feed(self, mut tx: mpsc::Sender<T>) -> io::Result<()> {
        let mut newest_item = {
            let endpoint = self
                .endpoint
                .clone()
                .filter(None, SearchSort::New, None, None);
            //Yikes
            self.reddit
                .app
                .create_request::<RedditResponseGeneric<SearchInfo<T>>>(endpoint.to_url())
                .await?
                .data
                .results
                .children
                .iter()
                .nth(0)
                .map(|e| e.data.feed_id())
        };

        loop {
            delay_for(self.delay).await;
            let before = if let Some(e) = &newest_item {
                Some(e.as_str())
            } else {
                None
            };

            let ep = self
                .endpoint
                .clone()
                .filter(None, SearchSort::New, before, None);

            let search = self
                .reddit
                .app
                .create_request::<RedditResponseGeneric<SearchInfo<T>>>(ep.to_url())
                .await
                .unwrap()
                .data
                .results
                .inner_children();
            if search.len() > 0 {
                newest_item = Some(search[0].feed_id().clone());
            }

            for item in search.iter().rev() {
                tx.send(item.clone())
                    .await
                    .map_err(|_| io::Error::new(io::ErrorKind::ConnectionReset, ""))?;
            }
        }
    }

    /// start polling the feed and return the new items.
    pub fn start(self) -> io::Result<mpsc::Receiver<T>> {
        let (tx, rx) = mpsc::channel(10);
        tokio::spawn(async { self.read_feed(tx).await });
        Ok(rx)
    }
}
