pub mod endpoints;
mod feed;
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
pub trait AbstractedApi<'r> {
    type ApiType: Clone + DeserializeOwned;
    type AbstractedType: 'r;
    fn from_parent(parent: &'r Reddit, info: Self::ApiType) -> Self::AbstractedType;

    fn list_of(parent: &'r Reddit, data: &[Self::ApiType]) -> Vec<Self::AbstractedType> {
        let mut out = Vec::new();
        for d in data {
            out.push(Self::from_parent(parent, d.to_owned()));
        }
        out
    }
}
