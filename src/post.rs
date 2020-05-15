use crate::models::{
    PostInfo,
};
use crate::reddit::Reddit;

use crate::ChildRedditItem;

pub struct Post<'r>{
    reddit: &'r Reddit,
    info: PostInfo,
}

impl<'r> ChildRedditItem<'r> for Post<'r> {
    type DataType = Post<'r>;
    type Metadata = PostInfo;

    fn from_parent(parent: &'r Reddit, info: Self::Metadata) -> Self{
        Self{
            reddit: parent,
            info: info,
        }
    }
}

impl<'r> Post<'r> {
    pub fn info(&self) -> &PostInfo {
        &self.info
    }
}
