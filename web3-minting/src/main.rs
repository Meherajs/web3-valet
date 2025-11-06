use std::net::SocketAddr;

mod blockchain;
mod handlers;
mod models;
mod storage;

use axum::{routing::post, Router};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    // Build our application with routes
    let app = Router::new().route("/mint", post(handlers::mint));

    // Run on 0.0.0.0:8081
    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    tracing::info!("Starting web3-minting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app).await.expect("Server failed");
}
