use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct User {
    pub name: String,
    pub token: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Event {
    pub uuid: String,
    pub title: String,
    pub description: String,
    pub date: String,
    pub time: String,
    pub location: String,
    pub organiser: String,
    pub contact_number: Option<String>,
    pub accepted: bool,
    pub accepted_by: Option<String>,
}
