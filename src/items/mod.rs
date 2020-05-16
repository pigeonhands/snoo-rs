
pub mod post;
pub mod search;
pub mod submission;
pub mod subreddit;
pub mod user;


pub use crate::endpoints::SearchSort;
pub use crate::reddit::Reddit;
use serde::de::DeserializeOwned;

// Represents something that is an abstraction over
// the raw api result model.
pub trait AbstractedApi<'r> {
    type ApiType: Clone + DeserializeOwned;
    type AbstractedType: 'r;
    fn from_parent(reddit: &'r Reddit, info: Self::ApiType) -> Self::AbstractedType;

    fn list_of(reddit: &'r Reddit, data: &[Self::ApiType]) -> Vec<Self::AbstractedType> {
        let mut out = Vec::new();
        for d in data {
            out.push(Self::from_parent(reddit, d.to_owned()));
        }
        out
    }
}
