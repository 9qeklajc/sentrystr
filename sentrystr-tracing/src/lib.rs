//! # SentryStr Tracing
//!
//! Tracing integration for SentryStr that seamlessly captures structured logs and sends them to Nostr relays.
//!
//! ## Quick Start
//!
//! ```rust
//! use sentrystr_tracing::SentryStrTracingBuilder;
//! use tracing::{info, warn, error};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let relays = vec!["wss://relay.damus.io".to_string()];
//!
//!     // Initialize tracing
//!     SentryStrTracingBuilder::new()
//!         .with_generated_keys_and_relays(relays)
//!         .with_min_level(tracing::Level::INFO)
//!         .init()
//!         .await?;
//!
//!     // Now all tracing events are sent to Nostr
//!     info!("Application started");
//!     warn!(cpu_usage = 85.5, "High CPU usage");
//!     error!(error_code = 500, "Database connection failed");
//!
//!     Ok(())
//! }
//! ```
//!
//! ## With Direct Message Alerts
//!
//! ```rust
//! use sentrystr_tracing::{SentryStrTracingBuilder, builder::DirectMessageConfig};
//! use sentrystr::Level;
//! use nostr::Keys;
//! use tracing::{info, error};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let relays = vec!["wss://relay.damus.io".to_string()];
//!     let recipient = Keys::generate().public_key();
//!
//!     // Setup DM alerts for errors
//!     let dm_config = DirectMessageConfig::new(recipient, relays.clone())
//!         .with_min_level(Level::Error)
//!         .with_nip17(true);
//!
//!     SentryStrTracingBuilder::new()
//!         .with_generated_keys_and_relays(relays)
//!         .with_direct_messaging(dm_config)
//!         .init()
//!         .await?;
//!
//!     info!("This won't send a DM");
//!     error!("This will send a DM!"); // This triggers a direct message
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Web Application Integration
//!
//! ```rust
//! use sentrystr_tracing::SentryStrTracingBuilder;
//! use tracing::{info, error, instrument};
//!
//! #[instrument]
//! async fn handle_request(user_id: u64) -> Result<String, String> {
//!     info!(user_id = user_id, "Processing request");
//!
//!     if user_id == 0 {
//!         error!(user_id = user_id, "Invalid user ID");
//!         return Err("Invalid user".to_string());
//!     }
//!
//!     Ok("Success".to_string())
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     SentryStrTracingBuilder::new()
//!         .with_generated_keys_and_relays(vec!["wss://relay.damus.io".to_string()])
//!         .init_with_env_filter("info,my_app=debug")
//!         .await?;
//!
//!     // All function calls are now traced
//!     let _ = handle_request(123).await;
//!     let _ = handle_request(0).await; // This will log an error
//!
//!     Ok(())
//! }
//! ```

pub mod builder;
pub mod error;
pub mod layer;
pub mod visitor;

pub use builder::SentryStrTracingBuilder;
pub use error::TracingError;
pub use layer::SentryStrLayer;
pub use visitor::FieldVisitor;

use sentrystr::{Event, Level};
use std::collections::BTreeMap;
use tracing::Metadata;

pub type Result<T> = std::result::Result<T, TracingError>;

pub fn convert_tracing_level(level: &tracing::Level) -> Level {
    match *level {
        tracing::Level::TRACE => Level::Debug,
        tracing::Level::DEBUG => Level::Debug,
        tracing::Level::INFO => Level::Info,
        tracing::Level::WARN => Level::Warning,
        tracing::Level::ERROR => Level::Error,
    }
}

pub fn extract_event_metadata(metadata: &Metadata<'_>) -> BTreeMap<String, serde_json::Value> {
    let mut fields = BTreeMap::new();

    fields.insert(
        "target".to_string(),
        serde_json::Value::String(metadata.target().to_string()),
    );
    fields.insert(
        "module_path".to_string(),
        metadata
            .module_path()
            .map(|s| serde_json::Value::String(s.to_string()))
            .unwrap_or(serde_json::Value::Null),
    );
    fields.insert(
        "file".to_string(),
        metadata
            .file()
            .map(|s| serde_json::Value::String(s.to_string()))
            .unwrap_or(serde_json::Value::Null),
    );
    fields.insert(
        "line".to_string(),
        metadata
            .line()
            .map(|n| serde_json::Value::Number(n.into()))
            .unwrap_or(serde_json::Value::Null),
    );
    fields.insert(
        "level".to_string(),
        serde_json::Value::String(metadata.level().to_string()),
    );

    fields
}

pub fn create_sentrystr_event(
    message: String,
    level: Level,
    fields: BTreeMap<String, serde_json::Value>,
    metadata_fields: BTreeMap<String, serde_json::Value>,
) -> Event {
    let mut event = Event::new().with_message(message).with_level(level);

    for (key, value) in fields {
        event = event.with_extra(&key, value);
    }

    for (key, value) in metadata_fields {
        event = event.with_extra(format!("meta_{}", key), value);
    }

    event
}
