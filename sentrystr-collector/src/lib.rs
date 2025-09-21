//! # SentryStr Collector
//!
//! Event collection and monitoring from Nostr relays with optional direct message alerting.
//!
//! ## Quick Start
//!
//! ```rust
//! use sentrystr_collector::{EventCollector, EventFilter};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let relays = vec!["wss://relay.damus.io".to_string()];
//!     let collector = EventCollector::new(relays).await?;
//!
//!     // Collect recent events
//!     let filter = EventFilter::new().with_limit(10);
//!     let events = collector.collect_events(filter).await?;
//!
//!     for event in events {
//!         println!("Event: {} from {}", event.event.message, event.author);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## With Private Messaging
//!
//! ```rust
//! use sentrystr_collector::{EventCollector, PrivateMessageConfig};
//! use sentrystr::Level;
//! use nostr::PublicKey;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let relays = vec!["wss://relay.damus.io".to_string()];
//!     let mut collector = EventCollector::new(relays).await?;
//!
//!     // Setup DM alerts for errors
//!     let recipient = PublicKey::from_bech32("npub1...")?;
//!     let dm_config = PrivateMessageConfig {
//!         recipient_pubkey: recipient,
//!         min_level: Some(Level::Error),
//!         use_nip17: true,
//!     };
//!
//!     collector = collector.with_private_messaging(dm_config)?;
//!
//!     // Subscribe to live events with DM alerts
//!     let mut rx = collector.subscribe_to_events(EventFilter::new()).await?;
//!     while let Some(event) = rx.recv().await {
//!         println!("Live event: {}", event.event.message);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Event Filtering
//!
//! ```rust
//! use sentrystr_collector::{EventCollector, EventFilter};
//! use sentrystr::Level;
//! use nostr::PublicKey;
//! use chrono::{Utc, Duration};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let relays = vec!["wss://relay.damus.io".to_string()];
//!     let collector = EventCollector::new(relays).await?;
//!
//!     let author = PublicKey::from_bech32("npub1...")?;
//!     let since = Utc::now() - Duration::hours(24);
//!
//!     let filter = EventFilter::new()
//!         .with_author(author)
//!         .with_level(Level::Error)
//!         .with_since(since)
//!         .with_limit(50);
//!
//!     let events = collector.collect_events(filter).await?;
//!     println!("Found {} error events in the last 24 hours", events.len());
//!
//!     Ok(())
//! }
//! ```

pub mod collector;
pub mod error;
pub mod filter;

pub use collector::{EventCollector, PrivateMessageConfig};
pub use error::CollectorError;
pub use filter::EventFilter;

pub type Result<T> = std::result::Result<T, CollectorError>;
