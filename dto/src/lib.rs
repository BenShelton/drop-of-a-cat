use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Video {
    pub id: u32,
    pub title: String,
    pub speaker: String,
    pub url: String,
}
