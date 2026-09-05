use axum::{
    extract::Request,
    routing::any,
    Router,
};
use reqwest::Client;

mod proxy;
mod config;

#[tokio::main]
async fn main() {

    let config = config::Config::from_file("config/proxy.toml")
        .expect("Failed to load configuration!");
    
    println!("Configuration loaded:");
    println!("{:#?}", config);
        
    let client = Client::new();

    let upstream = config.upstream.clone();

    let app = Router::new()
        .route("/{*path}", any(move |request: Request| {
            let client = client.clone();
            let upstream = upstream.clone();

            async move {
                proxy::proxy_request(
                        client,
                        upstream,
                        request,
                        ).await
            }
        }));

    let listener = tokio::net::TcpListener::bind(&config.listen)
        .await
        .unwrap();

    println!("Proxy running on http://{}",config.listen);

    axum::serve(listener, app)
        .await
        .unwrap();
}
