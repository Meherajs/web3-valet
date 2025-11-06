//! MCP API Server
//!
//! A REST API server that acts as a middleware layer between frontend clients and the
//! Model Context Protocol (MCP) server. This server handles text and audio inputs,
//! orchestrates AI agent interactions, and integrates with Speech-to-Text (STT) and
//! Text-to-Speech (TTS) services.
//!
//! # Architecture
//!
//! The server uses Axum framework with async/await patterns and shared state management
//! via `Arc<AppState>`. All external API calls use a shared HTTP client for efficient
//! connection pooling.
//!
//! # Endpoints
//!
//! - `GET /health` - Health check endpoint
//! - `GET /agents` - List all available agents from MCP
//! - `POST /input/text` - Process text input and return agent response with audio
//! - `POST /input/audio` - Process audio input, transcribe, and return agent response

use axum::{
    Router,
    response::IntoResponse,
    routing::{get, post},
};
use reqwest::Client;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

mod handlers;
mod models;

/// Application state shared across all request handlers.
///
/// This struct is wrapped in an `Arc` and cloned for each request handler,
/// providing thread-safe access to shared resources.
#[derive(Clone)]
struct AppState {
    /// Shared HTTP client for making requests to external services (MCP, ElevenLabs).
    /// Using a single client enables connection pooling and better performance.
    http_client: Client,
    /// ElevenLabs API key for STT and TTS operations.
    elevenlabs_api_key: String,
    /// Directory path where generated audio files are stored.
    audio_dir: String,
}

/// Main entry point for the MCP API server.
///
/// Initializes the server with:
/// - Environment variable loading from .env file
/// - Structured logging with tracing
/// - CORS middleware for cross-origin requests
/// - Shared application state
/// - Route definitions
///
/// # Panics
///
/// Panics if the server fails to bind to the specified address or if required
/// environment variables are not set.
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let elevenlabs_api_key = std::env::var("ELEVENLABS_API_KEY")
        .expect("ELEVENLABS_API_KEY must be set in .env file");
    
    let audio_dir = std::env::var("AUDIO_DIR").unwrap_or_else(|_| "public/audio".to_string());
    std::fs::create_dir_all(&audio_dir).expect("Failed to create audio directory");

    let shared_client = Client::new();
    
    let app_state = Arc::new(AppState {
        http_client: shared_client,
        elevenlabs_api_key,
        audio_dir: audio_dir.clone(),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/agents", get(handlers::get_agents_list))
        .route("/input/text", post(handlers::handle_text_input))
        .route("/input/audio", post(handlers::handle_audio_input))
        .nest_service("/public", ServeDir::new("public"))
        .layer(cors)
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::info!("Server listening on http://{}", addr);
    tracing::info!("Audio files will be stored in: {}", audio_dir);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

/// Health check endpoint handler.
///
/// Returns a simple "OK" response to indicate the server is running.
///
/// # Returns
///
/// Returns HTTP 200 with "OK" text response.
async fn health_check() -> impl IntoResponse {
    tracing::info!("Health check was accessed");
    "OK"
}
