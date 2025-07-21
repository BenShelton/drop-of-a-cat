use db::collections;
use dto::api::{APIError, EventResponse};
use http::Method;
use vercel_runtime::{
    http::{bad_request, not_found, ok},
    run, Body, Error, Request, Response,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    // TODO: Add authentication middleware
    assert_eq!(*req.method(), Method::GET);

    let uuid = req.uri().query().and_then(|query| {
        query
            .split('&')
            .find(|&param| param.starts_with("uuid="))
            .and_then(|param| param.split('=').nth(1))
    });

    let uuid = match uuid {
        Some(uuid) => uuid.to_string(),
        None => {
            return bad_request(APIError {
                message: "UUID query parameter is required.".to_string(),
                code: "missing_uuid".to_string(),
            });
        }
    };

    match collections::event::find_by_id(&uuid).await {
        Err(_) => not_found(APIError {
            message: "Failed to fetch event.".to_string(),
            code: "fetch_error".to_string(),
        }),
        Ok(event) => match event {
            None => not_found(APIError {
                message: "Event not found.".to_string(),
                code: "not_found".to_string(),
            }),
            Some(event) => ok(EventResponse { event }),
        },
    }
}
