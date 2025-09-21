//! # SentryStr API
//!
//! REST API server for SentryStr event collection and querying.
//!
//! ## Quick Start
//!
//! ```rust
//! use sentrystr_api::create_app;
//! use sentrystr_collector::EventCollector;
//! use std::sync::Arc;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let relays = vec!["wss://relay.damus.io".to_string()];
//!     let collector = Arc::new(EventCollector::new(relays).await?);
//!
//!     let app = create_app(collector);
//!
//!     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
//!     println!("SentryStr API server running on http://localhost:3000");
//!
//!     axum::serve(listener, app).await?;
//!     Ok(())
//! }
//! ```
//!
//! ## API Endpoints
//!
//! ### GET /events
//! Query events with optional filters:
//! ```bash
//! curl "http://localhost:3000/events?limit=10&level=error"
//! curl "http://localhost:3000/events?author=npub1...&since=2024-01-01T00:00:00Z"
//! ```
//!
//! ### GET /events/{author}
//! Get events by specific author:
//! ```bash
//! curl "http://localhost:3000/events/npub1abc123.../events"
//! ```
//!
//! ### GET /health
//! Health check endpoint:
//! ```bash
//! curl "http://localhost:3000/health"
//! ```
//!
//! ## With Tracing Integration
//!
//! ```rust
//! use sentrystr_api::create_app;
//! use sentrystr_collector::EventCollector;
//! use sentrystr_tracing::SentryStrTracingBuilder;
//! use tracing::{info, error};
//! use std::sync::Arc;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Setup tracing for the API server itself
//!     SentryStrTracingBuilder::new()
//!         .with_generated_keys_and_relays(vec!["wss://relay.damus.io".to_string()])
//!         .init()
//!         .await?;
//!
//!     let relays = vec!["wss://relay.damus.io".to_string()];
//!     let collector = Arc::new(EventCollector::new(relays).await?);
//!     let app = create_app(collector);
//!
//!     info!("Starting SentryStr API server");
//!
//!     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
//!     axum::serve(listener, app).await?;
//!
//!     Ok(())
//! }
//! ```

pub mod api;
pub mod handlers;
pub mod models;

pub use api::create_app;
pub use handlers::*;
pub use models::*;

pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    Collection(String),
    Internal(String),
    BadRequest(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Collection(msg) => write!(f, "Collection error: {}", msg),
            ApiError::Internal(msg) => write!(f, "Internal error: {}", msg),
            ApiError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}

impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            ApiError::Collection(msg) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::Internal(msg) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::BadRequest(msg) => (axum::http::StatusCode::BAD_REQUEST, msg),
        };

        let body = serde_json::json!({
            "error": error_message
        });

        (status, axum::Json(body)).into_response()
    }
}
