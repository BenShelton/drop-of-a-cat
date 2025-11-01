use dto::api::{APIError, HomeResponse, LoginRequest, LoginResponse};
use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};

pub const TOKEN_KEY: &str = "token";

pub fn get_authorization_header() -> String {
    LocalStorage::get(TOKEN_KEY).unwrap_or_default()
}

pub async fn login(request: &LoginRequest) -> Result<LoginResponse, String> {
    let result = Request::post("/api/login")
        .json(request)
        .unwrap()
        .send()
        .await
        .unwrap();
    if !result.ok() {
        return Err(result.json::<APIError>().await.unwrap().message.to_string());
    }
    Ok(result.json::<LoginResponse>().await.unwrap())
}

pub async fn home() -> Result<HomeResponse, String> {
    let auth_header = get_authorization_header();
    let result = Request::get("/api/home")
        .header("Authorization", auth_header.as_str())
        .send()
        .await
        .map_err(|_| "Failed to fetch events".to_string())?;
    if !result.ok() {
        return Err("Failed to fetch events".to_string());
    }
    result
        .json::<HomeResponse>()
        .await
        .map_err(|_| "Failed to fetch events".to_string())
}
