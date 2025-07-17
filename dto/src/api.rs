use serde::{Deserialize, Serialize};

use crate::collections::User;

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
pub struct APIError {
    pub message: String,
    pub code: String,
}
