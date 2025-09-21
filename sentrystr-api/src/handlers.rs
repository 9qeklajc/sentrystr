use axum::{extract::Query, Json};
use chrono::Utc;
use nostr::PublicKey;
use sentrystr_collector::{EventCollector, EventFilter};
use sentrystr::Level;

use crate::models::{EventQuery, EventResponse, EventsResponse, HealthResponse};
use crate::{ApiError, Result};

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        timestamp: Utc::now(),
    })
}

pub async fn get_events(Query(params): Query<EventQuery>) -> Result<Json<EventsResponse>> {
    let relays = vec!["wss://relay.damus.io".to_string()];

    let collector = EventCollector::new(relays)
        .await
        .map_err(|e| ApiError::Collection(e.to_string()))?;

    let mut filter = EventFilter::new();

    if let Some(limit) = params.limit {
        filter = filter.with_limit(limit);
    } else {
        filter = filter.with_limit(100);
    }

    if let Some(author_str) = params.author {
        let author = PublicKey::parse(&author_str)
            .map_err(|e| ApiError::BadRequest(format!("Invalid public key: {}", e)))?;
        filter = filter.with_author(author);
    }

    if let Some(level_str) = params.level {
        let level = match level_str.to_lowercase().as_str() {
            "debug" => Level::Debug,
            "info" => Level::Info,
            "warning" => Level::Warning,
            "error" => Level::Error,
            "fatal" => Level::Fatal,
            _ => return Err(ApiError::BadRequest("Invalid level".to_string())),
        };
        filter = filter.with_level(level);
    }

    if let Some(service) = params.service {
        filter = filter.with_service_filter(service);
    }

    if let Some(environment) = params.environment {
        filter = filter.with_environment_filter(environment);
    }

    if let Some(component) = params.component {
        filter = filter.with_component_filter(component);
    }

    if let Some(severity) = params.severity {
        filter = filter.with_severity_filter(severity);
    }

    if let Some(since) = params.since {
        filter = filter.with_since(since);
    }

    if let Some(until) = params.until {
        filter = filter.with_until(until);
    }

    let events = collector
        .collect_events(filter)
        .await
        .map_err(|e| ApiError::Collection(e.to_string()))?;

    collector
        .disconnect()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let response_events: Vec<EventResponse> = events
        .into_iter()
        .map(|event| EventResponse {
            nostr_event_id: event.nostr_event_id.to_string(),
            author: event.author.to_string(),
            received_at: event.received_at,
            event: crate::models::EventData {
                event_id: event.event.event_id,
                timestamp: event.event.timestamp,
                platform: event.event.platform,
                level: event.event.level,
                logger: event.event.logger,
                transaction: event.event.transaction,
                server_name: event.event.server_name,
                release: event.event.release,
                environment: event.event.environment,
                message: event.event.message,
                tags: event.event.tags,
                extra: event.event.extra,
            },
        })
        .collect();

    let total = response_events.len();

    Ok(Json(EventsResponse {
        events: response_events,
        total,
    }))
}
