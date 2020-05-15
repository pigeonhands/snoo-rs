use crate::reddit::Reddit;

use crate::models::{RedditResponseGeneric, SearchInfo, PostInfo};
use crate::ChildRedditItem;
use crate::{post::Post};

use crate::endpoints::SearchSort;

use crate::endpoints::Endpoint;

use std::io;


use std::sync::mpsc;
use tokio::task;
use tokio::time::{Duration, delay_for};

pub struct PostFeed{
    reddit: Reddit,
    endpoint: Endpoint,
}

impl PostFeed{
    pub fn new(reddit: Reddit, search_ep: Endpoint) -> PostFeed {
        PostFeed {
            reddit: reddit,
            endpoint: search_ep,
        }
    }

    async fn query<'a>(&'a self, before: &Option<String>) -> io::Result<Vec<Post<'a>>>{
        let ep = self.endpoint
            .as_filter_url::<String>(None, SearchSort::New, None, None)?;

        let search = self.reddit
            .create_request::<RedditResponseGeneric<SearchInfo<PostInfo>>>(ep)
            .await?
            .data;

        let results = {
            let result_info = search.results.inner_children();
            Post::list_of(&self.reddit, &result_info)
        };

        Ok(results)
    }

    async fn  run(self, tx: mpsc::Sender<PostInfo>) {
        println!("goo");
        let mut last_item = if let Some(item) = self.query(&None).await.unwrap().first() {
            Some(item.info().name.clone())
        }else{
            None
        };

        loop {
            let items = self.query(&last_item).await.unwrap();
            println!("Items: {} | Last: {:?}", items.len(), last_item);
            if items.len() > 0 {
                last_item = Some(items[0].info().name.clone());
            }
            
            for i in items {
                tx.send(i.info().clone()).unwrap()
            }

            delay_for(Duration::from_secs(3)).await;
        }
    }
    
    pub fn start(self) -> mpsc::Receiver<PostInfo> {
        println!("??");
        let (tx, rx) = mpsc::channel();
        
        tokio::spawn(async {
            println!("??1");
           self.run(tx).await
        });

        rx
    }
}