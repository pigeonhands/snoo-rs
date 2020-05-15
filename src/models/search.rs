
use crate::models::{
    ListingData,
};

use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone)]
pub struct SearchInfo<T>{
    #[serde(flatten)]
    pub results: ListingData<T>,
    pub after: Option<String>,
    pub before: Option<String>
}
