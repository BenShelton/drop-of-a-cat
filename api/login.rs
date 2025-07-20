use db::collections;
use dotenvy_macro::dotenv;
use dto::{
    api::{APIError, LoginRequest, LoginResponse},
    collections::User,
};
use http::Method;
use uuid::Uuid;
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
            message: "Invalid payload.".to_string(),
            code: "invalid_payload".to_string(),
        }),
        Ok(None) => bad_request(APIError {
            message: "No payload.".to_string(),
            code: "no_payload".to_string(),
        }),
        Ok(Some(payload)) => {
            if payload.name.is_empty() || payload.password.is_empty() {
                return bad_request(APIError {
                    message: "Name and password are required.".to_string(),
                    code: "missing_fields".to_string(),
                });
            }
            ok(LoginResponse {
                result: if payload.password == dotenv!("SITE_PASSWORD") {
                    let user = User {
                        name: payload.name,
                        token: Uuid::new_v4().to_string(),
                    };
                    match collections::user::insert(&user).await {
                        Ok(()) => Some(user),
                        Err(_) => None,
                    }
                } else {
                    None
                },
            })
        }
    }
}
