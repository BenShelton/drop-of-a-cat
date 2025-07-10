use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginRequest {
    pub name: String,
    pub password: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginResponse {
    pub result: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct APIError {
    pub message: &'static str,
    pub code: &'static str,
}
