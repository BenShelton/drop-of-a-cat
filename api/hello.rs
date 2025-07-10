use dto::Video;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let response = serde_json::to_string(&Video {
        id: 1,
        title: "Hello World".to_string(),
        speaker: "Yew Developer".to_string(),
        url: "https://example.com/video.mp4".to_string(),
    })?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(response.into())?)
}
