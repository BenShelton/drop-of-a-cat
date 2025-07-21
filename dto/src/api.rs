use serde::{Deserialize, Serialize};

use crate::collections::{Event, User};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginRequest {
    pub name: String,
    pub password: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginResponse {
    pub result: Option<User>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct SuggestionRequest {
    pub event: Event,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct SuggestionResponse {
    pub result: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct HomeResponse {
    pub events: Vec<Event>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct EventResponse {
    pub event: Event,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct APIError {
    pub message: String,
    pub code: String,
}
