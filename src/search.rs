use crate::reddit::Reddit;

use crate::models::{RedditResponseGeneric, SearchInfo};
use crate::ChildRedditItem;
use crate::{post::Post, subreddit::Subreddit};

use crate::endpoints::SearchSort;

use crate::endpoints::Endpoint;

use std::io;
use std::rc::Rc;

pub type PostSearch<'r, 's> = RedditSearch<'r, 's, Post<'r>>;
pub type SubredditSearch<'r, 's> = RedditSearch<'r, 's, Subreddit<'r>>;
pub type UserSearch<'r, 's> = RedditSearch<'r, 's, Subreddit<'r>>;

struct SearchParams<'r, 's> {
    reddit: &'r Reddit,
    query: &'s str,
    sort: SearchSort,
    endpoint: Endpoint,
}

pub struct RedditSearch<'r, 's, T: ChildRedditItem<'r>> {
    params: Rc<SearchParams<'r, 's>>,
    results: Vec<T::DataType>,
    before: Option<String>,
    after: Option<String>,
}

impl<'r, 's, T: ChildRedditItem<'r>> RedditSearch<'r, 's, T> {
    pub(crate) async fn new_search(
        parent: &'r Reddit,
        search_ep: Endpoint,
        query: &'s str,
        sort: SearchSort,
    ) -> io::Result<RedditSearch<'r, 's, T>> {
        let params = Rc::from(SearchParams {
            reddit: parent,
            query: query,
            sort: sort,
            endpoint: search_ep,
        });

        Self::search(params, None, None).await
    }

    async fn search(
        params: Rc<SearchParams<'r, 's>>,
        before: Option<&str>,
        after: Option<&str>,
    ) -> io::Result<RedditSearch<'r, 's, T>> {
        let ep = params
            .endpoint
            .as_search_url(params.query, params.sort, before, after)?;

        let search = params
            .reddit
            .create_request::<RedditResponseGeneric<SearchInfo<T::Metadata>>>(ep)
            .await?
            .data;

        let results = {
            let result_info = search.results.inner_children();
            T::list_of(params.reddit, &result_info)
        };

        Ok(RedditSearch::<'r, 's> {
            params: params,
            results: results,
            before: search.before,
            after: search.after,
        })
    }

    /// Current search results
    pub fn results(&self) -> &Vec<T::DataType> {
        &self.results
    }

    /// Next page of results
    pub async fn next(&self) -> io::Result<Option<RedditSearch<'r, 's, T>>> {
        Ok(if let Some(next) = &self.after {
            Some(Self::search(self.params.clone(), None, Some(next)).await?)
        } else {
            None
        })
    }

    /// Previous page of results
    pub async fn prev(&self) -> Option<io::Result<RedditSearch<'r, 's, T>>> {
        if let Some(prev) = &self.before {
            Some(Self::search(self.params.clone(), Some(prev), None).await)
        } else {
            None
        }
    }
}
