use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

use crate::handlers::{get_events, health};

pub fn create_app() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/events", get(get_events))
        .layer(CorsLayer::permissive())
}
