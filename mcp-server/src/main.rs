//! MCP Server (Model Context Protocol)
//!
//! A JSON-RPC 2.0 server that provides AI agent functionality using Google's Gemini API.
//! This server manages multiple specialized AI agents and processes user requests with
//! context-aware responses.
//!
//! # Architecture
//!
//! The server is organized into several modules:
//! - `models` - Data structures for JSON-RPC, agents, and Gemini API
//! - `agents` - Agent definitions and management
//! - `gemini` - Gemini API client and communication
//! - `handlers` - HTTP request handlers for JSON-RPC methods
//!
//! # Supported Methods
//!
//! - `list_agents` - Returns all available AI agents
//! - `process_text` - Processes user text through a specified agent
//!
//! # Quick Start
//!
//! 1. Set `GEMINI_API_KEY` in your `.env` file
//! 2. Run `cargo run --release`
//! 3. Server starts on `http://0.0.0.0:3000`
//! 4. Send JSON-RPC 2.0 requests to the root path

mod agents;
mod gemini;
mod handlers;
mod models;

use axum::{routing::post, Router};
use reqwest::Client;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Application state shared across all request handlers.
///
/// This struct is wrapped in an `Arc` and cloned for each request handler,
/// providing thread-safe access to shared resources.
#[derive(Clone)]
pub struct AppState {
    /// Shared HTTP client for making requests to Gemini API.
    pub http_client: Client,
    /// Google Gemini API key for authentication.
    pub gemini_api_key: String,
}

/// Main entry point for the MCP server.
///
/// Initializes the server with:
/// - Environment variable loading from .env file
/// - Structured logging with tracing
/// - CORS middleware for cross-origin requests
/// - Shared application state with Gemini API key
/// - JSON-RPC route at the root path
///
/// # Environment Variables
///
/// * `GEMINI_API_KEY` - Required. Google Gemini API key for agent responses
/// * `RUST_LOG` - Optional. Logging level (default: info)
///
/// # Panics
///
/// Panics if:
/// - GEMINI_API_KEY is not set
/// - Server fails to bind to port 3000
#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize structured logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "mcp_server=debug,tower_http=debug,axum=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load Gemini API key from environment
    let gemini_api_key =
        std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set in .env file");

    // Create shared HTTP client
    let http_client = Client::new();

    // Create shared application state
    let state = Arc::new(AppState {
        http_client,
        gemini_api_key,
    });

    // Build the router with CORS support
    let app = Router::new()
        .route("/", post(handlers::handle_jsonrpc))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Bind to TCP listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    // Log startup information
    tracing::info!("🚀 MCP Server starting on http://0.0.0.0:3000");
    tracing::info!("📋 Available agents: {}", agents::get_agents().len());
    tracing::info!("🤖 Using Google Gemini for agent responses");
    tracing::info!("📡 Supported JSON-RPC methods:");
    tracing::info!("   - list_agents");
    tracing::info!("   - process_text");

    // Start the server
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
