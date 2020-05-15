use crate::models::{
    RedditResponse,
    PostInfo,
    ListingData,
    CommentData
};

use crate::reddit::Reddit;
use crate::post::Post;
use crate::ChildRedditItem;

use crate::user::RedditUser;

/// Post + Comments
pub struct Submission<'r>{
    op: Post<'r>,
    comments: Vec<Comment<'r>>
}

impl<'r> Submission<'r> {

    pub (crate) fn from_resp(parent:&'r Reddit, listing: ListingData<RedditResponse>) -> Self {
        
        let mut op = PostInfo::default();
        let mut comments = Vec::new();
        
        for l in listing.children {
            match l.data {
                RedditResponse::Post(post) => op = post,
                RedditResponse::Comment(c) => comments.push(Comment::from_data(parent, c)),
                _ =>{}
            }
        }

        Self{
            op: Post::from_parent(parent, op),
            comments: comments
        }
    }

    pub fn op(&self) -> &Post {
        &self.op
    }

    pub fn comments(&self) -> &[Comment] {
        &self.comments
    }
}

/// A user comment
pub struct Comment<'r>{
   reddit: &'r Reddit,
   data: CommentData
}

impl<'r> Comment<'r>{
    fn from_data(parent:&'r Reddit, data: CommentData) -> Self{
        Self{
            reddit: parent,
            data:data,
        }
    }

    pub fn info(&self) -> &CommentData {
        &self.data
    }

    pub fn author(&self) -> RedditUser{
        self.reddit.user(&self.data.author)
    }
}


impl<'r> ChildRedditItem<'r> for Comment<'r> {
    type DataType = Comment<'r>;
    type Metadata = CommentData;

    fn from_parent(parent: &'r Reddit, info: Self::Metadata) -> Self{
        Self{
            reddit: parent,
            data: info,
        }
    }
}