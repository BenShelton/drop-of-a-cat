use api_utils::authorize_middleware;
use db::collections;
use dto::api::{APIError, HomeResponse};
use http::Method;
use vercel_runtime::{
    http::{not_found, ok},
    run, Body, Error, Request, Response,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    assert_eq!(*req.method(), Method::GET);
    if let Some(response) = authorize_middleware(&req).await {
        return Ok(response);
    };

    match collections::event::list().await {
        Err(_) => not_found(APIError {
            message: "Failed to fetch events.".to_string(),
            code: "fetch_error".to_string(),
        }),
        Ok(events) => {
            if events.is_empty() {
                return not_found(APIError {
                    message: "No events found.".to_string(),
                    code: "no_events".to_string(),
                });
            }

            ok(HomeResponse { events })
        }
    }
}
