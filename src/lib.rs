pub mod endpoints;
mod models;
pub mod post;
pub mod reddit;
pub mod search;
pub mod submission;
pub mod subreddit;
pub mod user;

pub use endpoints::SearchSort;
pub use reddit::Reddit;
use serde::de::DeserializeOwned;

// Represents something that is an abstraction over
// the raw api result model.
pub trait ChildRedditItem<'r> {
    type Metadata: Clone + DeserializeOwned;
    type DataType: 'r;
    fn from_parent(parent: &'r Reddit, info: Self::Metadata) -> Self::DataType;

    fn list_of(parent: &'r Reddit, data: &[Self::Metadata]) -> Vec<Self::DataType> {
        let mut out = Vec::new();
        for d in data {
            out.push(Self::from_parent(parent, d.to_owned()));
        }
        out
    }
}
