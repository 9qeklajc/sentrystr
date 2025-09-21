//! # SentryStr Core
//!
//! Core functionality for SentryStr - a decentralized error tracking and alerting system using Nostr.
//!
//! ## Quick Start
//!
//! ```rust
//! use sentrystr::{Config, Event, Level, NostrSentryClient};
//! use nostr::Keys;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Basic setup
//!     let keys = Keys::generate();
//!     let relays = vec!["wss://relay.damus.io".to_string()];
//!     let config = Config::new(keys.secret_key().display_secret().to_string(), relays);
//!
//!     // Create client
//!     let client = NostrSentryClient::new(config).await?;
//!
//!     // Send events
//!     let event = Event::new()
//!         .with_message("Something went wrong")
//!         .with_level(Level::Error);
//!
//!     client.capture_event(event).await?;
//!     client.capture_error("Database connection failed").await?;
//!     client.capture_message("System started").await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## With Direct Messaging
//!
//! ```rust
//! use sentrystr::{Config, DirectMessageBuilder, Event, Level, NostrSentryClient};
//! use nostr::prelude::*;
//! use nostr_sdk::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Setup main client
//!     let keys = Keys::generate();
//!     let relays = vec!["wss://relay.damus.io".to_string()];
//!     let config = Config::new(keys.secret_key().display_secret().to_string(), relays.clone());
//!     let mut client = NostrSentryClient::new(config).await?;
//!
//!     // Setup direct messaging
//!     let dm_keys = Keys::generate();
//!     let dm_client = Client::new(dm_keys.clone());
//!     dm_client.add_relay("wss://relay.damus.io").await?;
//!     dm_client.connect().await;
//!
//!     let recipient = Keys::generate().public_key();
//!     let dm_sender = DirectMessageBuilder::new()
//!         .with_client(dm_client)
//!         .with_keys(dm_keys)
//!         .with_recipient(recipient)
//!         .with_min_level(Level::Error)
//!         .with_nip17(true)
//!         .build()?;
//!
//!     client.set_direct_messaging(dm_sender);
//!
//!     // Now errors will also send DMs
//!     client.capture_error("Critical system failure").await?;
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod combined_example;
pub mod config;
pub mod encryption;
pub mod error;
pub mod event;
pub mod messaging;

pub use client::NostrSentryClient;
pub use config::{Config, EncryptionVersion};
pub use encryption::{validate_encryption_keys, EncryptionHelper};
pub use error::SentryStrError;
pub use event::{Event, Exception, Frame, Level, Request, Stacktrace, User};
pub use messaging::{DirectMessageBuilder, DirectMessageConfig, DirectMessageSender, MessageEvent};

pub type Result<T> = std::result::Result<T, SentryStrError>;
