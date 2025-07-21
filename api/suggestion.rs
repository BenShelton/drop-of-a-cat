use db::collections;
use dto::api::{APIError, SuggestionRequest, SuggestionResponse};
use http::Method;
use vercel_runtime::{
    http::{bad_request, ok},
    run, Body, Error, Request, RequestPayloadExt, Response,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    // TODO: Add authentication middleware
    assert_eq!(*req.method(), Method::POST);

    let payload = req.payload::<SuggestionRequest>();

    match payload {
        Err(..) => bad_request(APIError {
            message: "Invalid payload.".to_string(),
            code: "invalid_payload".to_string(),
        }),
        Ok(None) => bad_request(APIError {
            message: "No payload.".to_string(),
            code: "no_payload".to_string(),
        }),
        Ok(Some(payload)) => {
            if payload.event.title.is_empty() {
                return bad_request(APIError {
                    message: "Missing required fields.".to_string(),
                    code: "missing_fields".to_string(),
                });
            }
            let mut event = payload.event;
            ok(SuggestionResponse {
                result: collections::event::insert(&mut event).await.is_ok(),
            })
        }
    }
}
