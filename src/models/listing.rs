use crate::models::{
    RedditResponseGeneric,
};
use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone)]
pub struct ListingData<T> {
    pub modhash: String,
    pub dist: i32,
    pub children: Vec<RedditResponseGeneric<T>>
}


impl<T> ListingData<T> {
    pub fn inner_children(self) -> Vec<T> {
        let mut out = Vec::new();
        for c in self.children {
            out.push(c.data);
        }
        out
    }
}