use axum::{
    extract::Request,
    routing::any,
    Router,
};
use reqwest::Client;

mod proxy;

#[tokio::main]
async fn main() {
    let client = Client::new();

    let app = Router::new()
        .route("/{*path}", any(move |request: Request| {
            let client = client.clone();

            async move {
                proxy::handler::proxy_request(client, request).await
            }
        }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("Proxy running on http://localhost:8080");

    axum::serve(listener, app)
        .await
        .unwrap();
}
