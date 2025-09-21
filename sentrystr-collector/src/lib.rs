//! # SentryStr Collector
//!
//! Event collection and monitoring from Nostr relays with optional direct message alerting.
//!
//! ## Quick Start
//!
//! ```rust
//! use sentrystr_collector::{EventCollector, EventFilter};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let relays = vec!["wss://relay.damus.io".to_string()];
//!     // let collector = EventCollector::new(relays).await?;
//!     // let filter = EventFilter::new().with_limit(10);
//!     // let events = collector.collect_events(filter).await?;
//!     println!("Would collect events from {} relays", relays.len());
//!     Ok(())
//! # }
//! ```
//!
//! ## With Private Messaging
//!
//! ```rust
//! use sentrystr_collector::{EventCollector, PrivateMessageConfig, EventFilter};
//! use sentrystr::Level;
//! use nostr::Keys;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let relays = vec!["wss://relay.damus.io".to_string()];
//!     // let mut collector = EventCollector::new(relays).await?;
//!     let recipient = Keys::generate().public_key();
//!     println!("Would setup DM alerts to {}", recipient.to_hex());
//!     Ok(())
//! # }
//! ```
//!
//! ## Event Filtering
//!
//! ```rust
//! use sentrystr_collector::{EventCollector, EventFilter};
//! use sentrystr::Level;
//! use nostr::Keys;
//! use chrono::{Utc, Duration};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let relays = vec!["wss://relay.damus.io".to_string()];
//!     // let collector = EventCollector::new(relays).await?;
//!     let author = Keys::generate().public_key();
//!     let since = Utc::now() - Duration::hours(24);
//!     println!("Would filter events from {} since {}", author.to_hex(), since);
//!     Ok(())
//! # }
//! ```

pub mod collector;
pub mod error;
pub mod filter;

pub use collector::{EventCollector, PrivateMessageConfig};
pub use error::CollectorError;
pub use filter::EventFilter;

pub type Result<T> = std::result::Result<T, CollectorError>;
