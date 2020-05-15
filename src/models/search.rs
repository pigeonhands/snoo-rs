
use crate::models::{
    ListingData,
    PostInfo
};

use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone)]
pub struct SearchInfo{
    #[serde(flatten)]
    pub results: ListingData<PostInfo>,
    pub after: Option<String>,
    pub before: Option<String>
}
