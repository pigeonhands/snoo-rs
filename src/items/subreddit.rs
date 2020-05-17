use crate::endpoints::{self, SearchSort};
use crate::feed::ContentStream;
use crate::reddit::Reddit;

use crate::items::{post::Post, search::PostSearch, AbstractedApi};
use crate::models::{
    PostInfo, 
    SubredditInfo, 
    SubredditSubmit,
    SubredditSubmitResponse, 
    SubredditSubmitLink, 
    SubredditSubmitText
};


use reqwest::Url;

use chrono::{prelude::*, DateTime, Utc};

use std::io;

pub enum SubredditSubmission<'a>{
    Link(&'a str),
    Text(&'a str)
}

impl SubredditSubmission<'_>{
    pub fn kind(&self) -> &'static str {
        match self{
            SubredditSubmission::Link(_) => "link",
            SubredditSubmission::Text(_) => "self",
        }
    }
}

pub struct SubredditLink<'r> {
    pub reddit: &'r Reddit,
    pub subreddit: String,
}

impl<'r> SubredditLink<'r> {
    pub fn new(reddit: &'r Reddit, subreddit: &str) -> SubredditLink<'r> {
        SubredditLink {
            reddit: reddit,
            subreddit: subreddit.to_owned(),
        }
    }

    pub async fn get(self) -> io::Result<Subreddit<'r>> {
        let ep = self
            .reddit
            .ep(endpoints::SUBREDDIT_ABOUT.subreddit(&self.subreddit))?;
        let info = self.reddit.get_data::<SubredditInfo>(ep).await?;

        Ok(Subreddit {
            link: self,
            info: info.data,
        })
    }

    pub fn name(&self) -> &str {
        &self.subreddit
    }

    /// Stream of new posts in the subreddit.
    pub fn feed(&self) -> io::Result<ContentStream<PostInfo>> {
        let ep = self
            .reddit
            .ep(endpoints::SUBREDDIT_NEW.subreddit(self.name()))?;
        Ok(ContentStream::new(self.reddit.clone(), ep))
    }

    pub async fn top(&self) -> io::Result<Vec<Post<'r>>> {
        let ep = self
            .reddit
            .ep(endpoints::SUBREDDIT_TOP.subreddit(&self.subreddit))?;
        Ok(Post::list_of(self.reddit, &self.reddit.get_list(ep).await?))
    }

    pub async fn search<'s>(
        &'r self,
        query: &'s str,
        sort: SearchSort,
    ) -> io::Result<PostSearch<'r, 's>> {
        let search_ep = self
            .reddit
            .ep(endpoints::SUBREDDIT_SEARCH.subreddit(&self.subreddit))?;
        PostSearch::new_search(self.reddit, search_ep, query, sort).await
    }


    pub async fn submit(&self, title: &str, submission: SubredditSubmission<'_>) -> io::Result<SubredditSubmitResponse> {
        let submit = SubredditSubmit {
            kind: submission.kind(),
            sr: self.name(),
            title: title,
            resubmit: true,
            iden: None,
            captcha: None,
        };
       
        let target_url = self.reddit.ep(endpoints::SUBMIT.subreddit(&self.name()))?;
       
        match submission {
            SubredditSubmission::Link(link) => self.reddit.post_data::<_, SubredditSubmitResponse>(target_url,  &SubredditSubmitLink {
                submit: submit,
                url: link
            }).await,
            SubredditSubmission::Text(body) =>  self.reddit.post_data::<_, SubredditSubmitResponse>(target_url, &SubredditSubmitText {
                submit: submit,
                text: body   
            }).await
        }
    }

}

pub struct Subreddit<'r> {
    pub link: SubredditLink<'r>,
    pub info: SubredditInfo,
}

impl<'r> Subreddit<'r> {
    /// Returns the underlying [SubredditInfo] model.
    pub fn info(&self) -> &SubredditInfo {
        &self.info
    }

    pub fn name(&self) -> &str {
        self.link.name()
    }

    pub fn title(&self) -> &str {
        &self.info.title
    }

    pub fn subscribers(&self) -> Option<i32> {
        self.info.subscribers
    }

    pub async fn top(&self) -> io::Result<Vec<Post<'r>>> {
        self.link.top().await
    }

    pub async fn search<'s>(
        &'r self,
        query: &'s str,
        sort: SearchSort,
    ) -> io::Result<PostSearch<'r, 's>> {
        self.link.search(query, sort).await
    }

    pub fn url(&self) -> io::Result<Url> {
        Url::parse(&self.info.url)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse url"))
    }

    pub fn created(&self) -> Option<DateTime<Utc>> {
        if let Some(created) = self.info.created {
            let naive_datetime = NaiveDateTime::from_timestamp(created as i64, 0);
            Some(DateTime::from_utc(naive_datetime, Utc))
        } else {
            None
        }
    }

    pub async fn submit_text(&self, title: &str, body: &str) -> io::Result<SubredditSubmitResponse> {
        self.link.submit(title, SubredditSubmission::Text(body)).await
    }
}

impl<'r> AbstractedApi<'r> for Subreddit<'r> {
    type AbstractedType = Subreddit<'r>;
    type ApiType = SubredditInfo;

    fn from_parent(reddit: &'r Reddit, info: Self::ApiType) -> Subreddit<'r> {
        Subreddit {
            link: SubredditLink::new(reddit, &info.display_name),
            info,
        }
    }
}
