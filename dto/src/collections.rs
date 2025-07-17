use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub token: String,
}
