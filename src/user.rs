
use crate::models::{
    UserInfo,
};
use crate::reddit::Reddit;
use crate::endpoints;

use std::io;

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
}


pub struct UserOverview{

}