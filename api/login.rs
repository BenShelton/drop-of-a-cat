use api::APIError;
use dotenvy_macro::dotenv;
use dto::{LoginRequest, LoginResponse};
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
    assert_eq!(*req.method(), Method::POST);

    let payload = req.payload::<LoginRequest>();

    match payload {
        Err(..) => bad_request(APIError {
            message: "Invalid payload",
            code: "invalid_payload",
        }),
        Ok(None) => bad_request(APIError {
            message: "No payload",
            code: "no_payload",
        }),
        Ok(Some(payload)) => ok(LoginResponse {
            result: payload.password == dotenv!("SITE_PASSWORD"),
        }),
    }
}
