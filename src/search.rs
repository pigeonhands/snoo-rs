
use crate::reddit::Reddit;

use crate::post::Post;
use crate::models::{
    RedditResponseGeneric,
    SearchInfo
};
use crate::ChildRedditItem;

use crate::endpoints::{
    self,
    SearchSort,
};

use std::io;
use std::rc::Rc;

struct SearchParams<'r, 's>{
    reddit: &'r Reddit,
    query: &'s str,
    sort: SearchSort,
}

pub struct SearchResults<'r, 's>{
    params: Rc<SearchParams<'r, 's>>,
    posts: Vec<Post<'r>>,
    before: Option<String>,
    after: Option<String>,
}

impl<'r, 's> SearchResults<'r, 's> {
    pub (crate) async fn new_search(parent: &'r Reddit, query: &'s str, sort: SearchSort) -> io::Result<SearchResults<'r, 's>> {

        let params = Rc::from(SearchParams{
            reddit: parent,
            query: query,
            sort: sort,
        });

        Self::search(params, None, None).await
    }

    async fn search(params: Rc<SearchParams<'r,'s>>, before: Option<&str>, after: Option<&str>) ->  io::Result<SearchResults<'r, 's>> {
        let ep = endpoints::SEARCH.as_search_url(params.query, params.sort, before, after)?;
        let search = params.reddit.create_request::<RedditResponseGeneric<SearchInfo>>(ep).await?.data;
        
        let posts = {
            let post_info = search.results.inner_children();
            Post::list_of(params.reddit, &post_info)
        };

        Ok(SearchResults::<'r, 's>{
            params: params,
            posts: posts,
            before: search.before,
            after: search.after
        })
    }

    pub fn results(&self) -> &Vec<Post> {
        &self.posts
    }

    pub async fn next(&self) -> io::Result<Option<SearchResults<'r, 's>>> {
        
        Ok(if let Some(next) = &self.after {
            Some(Self::search(self.params.clone(), None, Some(next)).await?)
        }else{
            None
        })
    }

    pub async fn prev(&self) -> Option<io::Result<SearchResults<'r, 's>>> {
        if let Some(prev) = &self.before {
            Some(Self::search(self.params.clone(), Some(prev), None).await)
        }else{
            None
        }
    }
}