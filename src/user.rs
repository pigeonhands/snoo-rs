
use crate::models::{
    UserInfo,
};
use crate::{
    reddit::Reddit,
    endpoints,
    post::Post,
    submission::Comment,
};

use std::io;
use crate::ChildRedditItem;

pub struct RedditUser<'r> {
    reddit: &'r Reddit,
    name: String,
}


impl<'r> RedditUser<'r>{
    pub (crate) fn from_name(parent: &'r Reddit, name: &str) -> Self{
        Self{
            reddit: parent,
            name: name.to_owned(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub async fn about(&self) -> io::Result<UserInfo> {
        let ep = endpoints::USER_ABOUT.user(&self.name).as_api_endpoint()?;

        let about = self.reddit.get_data::<UserInfo>(ep).await?;
        Ok(about.data)
    }

    pub async fn submitted(&self) -> io::Result<Vec<Post<'r>>>{
        let ep = endpoints::USER_SUBMITTED.user(&self.name);
        Ok(Post::list_of(self.reddit, &self.reddit.get_list(ep).await?))
    }

    pub async fn comments(&self) -> io::Result<Vec<Comment<'r>>>{
        let ep = endpoints::USER_COMMENTS.user(&self.name);
        Ok(Comment::list_of(self.reddit, &self.reddit.get_list(ep).await?))
    }
}


pub struct UserOverview{

}