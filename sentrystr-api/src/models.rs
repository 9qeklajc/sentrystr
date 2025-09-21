use chrono::{DateTime, Utc};
use sentrystr::Level;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EventResponse {
    pub nostr_event_id: String,
    pub author: String,
    pub received_at: DateTime<Utc>,
    pub event: EventData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventData {
    pub event_id: String,
    pub timestamp: DateTime<Utc>,
    pub platform: String,
    pub level: Level,
    pub logger: Option<String>,
    pub transaction: Option<String>,
    pub server_name: Option<String>,
    pub release: Option<String>,
    pub environment: Option<String>,
    pub message: Option<String>,
    pub tags: std::collections::HashMap<String, String>,
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct EventQuery {
    pub author: Option<String>,
    pub level: Option<String>,
    pub service: Option<String>,
    pub environment: Option<String>,
    pub component: Option<String>,
    pub severity: Option<String>,
    pub since: Option<DateTime<Utc>>,
    pub until: Option<DateTime<Utc>>,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct EventsResponse {
    pub events: Vec<EventResponse>,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
}
