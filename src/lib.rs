pub mod endpoints;
pub mod reddit;
pub mod subreddit;
pub mod post;
pub mod submission;
pub mod search;
pub mod user;
mod models;


pub use reddit::Reddit;
pub use endpoints::SearchSort;
use serde::de::DeserializeOwned;

pub (crate) trait ChildRedditItem<'r> {
    type Metadata: Clone+DeserializeOwned;
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
