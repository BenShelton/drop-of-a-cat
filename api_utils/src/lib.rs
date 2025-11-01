use db::collections;
use vercel_runtime::{http::bad_request, Body, Request, Response};

/// Middleware to authorize requests based on the "Authorization" header.
///
/// # Arguments
/// * `req` - A reference to the incoming HTTP request.
///
/// # Returns
/// * `Option<Response<Body>>` - `None` if authorized, otherwise a response with an error message.
///
/// # Example
/// ```rust
/// if let Some(response) = authorize_middleware(&req).await {
///     return Ok(response);
/// }
/// ```
pub async fn authorize_middleware(req: &Request) -> Option<Response<Body>> {
    let Some(Ok(token)) = req.headers().get("Authorization").map(|h| h.to_str()) else {
        return Some(bad_request("Missing Authorization Token").unwrap());
    };
    match collections::user::validate(token).await {
        Ok(true) => None,
        _ => Some(bad_request("Bad Authorization Token").unwrap()),
    }
}
