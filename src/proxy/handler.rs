use axum::{
    body::{to_bytes, Body},
    extract::Request,
    http::{Response},
};
use reqwest::Client;

pub async fn proxy_request(
    client: Client,
    upstream: String,
    request: Request,
) -> Result<Response<Body>, String> {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let headers = request.headers().clone();

    let url = format!(
        "{}{}",
        upstream,
        uri,
    );

    let body = to_bytes(request.into_body(), usize::MAX)
        .await
        .map_err(|err| err.to_string())?;

    let response = client
        .request(method, url)
        .headers(headers)
        .body(body)
        .send()
        .await
        .map_err(|err| err.to_string())?;

    let status = response.status();
    let headers = response.headers().clone();

    let body = response
        .bytes()
        .await
        .map_err(|err| err.to_string())?;

    let mut builder = Response::builder()
        .status(status);

    for (name, value) in headers.iter() {
        builder = builder.header(name, value);
    }

    builder
        .body(Body::from(body))
        .map_err(|err| err.to_string())
}
