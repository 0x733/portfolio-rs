mod routes;
mod templates;

use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // Initialize tracing/logging for production-ready idiomatic Rust
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(routes::home_handler))
        .route("/about", get(routes::about_handler))
        .route("/project", get(routes::project_handler))
        .fallback_service(ServeDir::new("static"));

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    tracing::info!("Listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
