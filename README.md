# SentryStr

A decentralized error tracking and alerting system using Nostr relays. Send structured error events to Nostr with optional direct message alerts for critical issues.

## 🚀 Quick Start

### Core Usage (Direct Event Sending)

```rust
use sentrystr_core::{Config, Event, Level, NostrSentryClient};
use nostr::Keys;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let keys = Keys::generate();
    let relays = vec!["wss://relay.damus.io".to_string()];
    let config = Config::new(keys.secret_key().display_secret().to_string(), relays);

    let client = NostrSentryClient::new(config).await?;

    // Send different types of events
    client.capture_message("Application started").await?;
    client.capture_error("Database connection failed").await?;

    let custom_event = Event::new()
        .with_message("User action completed")
        .with_level(Level::Info)
        .with_tag("user_id", "12345")
        .with_extra("action", serde_json::json!("login"));

    client.capture_event(custom_event).await?;
    Ok(())
}
```

### Tracing Integration (Automatic Logging)

```rust
use sentrystr_tracing::SentryStrTracingBuilder;
use tracing::{info, warn, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing - all logs now go to Nostr
    SentryStrTracingBuilder::new()
        .with_generated_keys_and_relays(vec!["wss://relay.damus.io".to_string()])
        .with_min_level(tracing::Level::INFO)
        .init()
        .await?;

    // Normal tracing - automatically sent to Nostr
    info!("Application started");
    warn!(cpu_usage = 85.5, "High CPU usage detected");
    error!(error_code = 500, component = "database", "Connection failed");

    Ok(())
}
```

### Event Collection (Monitor Others)

```rust
use sentrystr_collector::{EventCollector, EventFilter};
use sentrystr_core::Level;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let relays = vec!["wss://relay.damus.io".to_string()];
    let collector = EventCollector::new(relays).await?;

    // Collect recent error events
    let filter = EventFilter::new()
        .with_level(Level::Error)
        .with_limit(10);

    let events = collector.collect_events(filter).await?;

    for event in events {
        println!("Error from {}: {}", event.author, event.event.message);
    }

    Ok(())
}
```

### API Server (HTTP Interface)

```rust
use sentrystr_api::create_app;
use sentrystr_collector::EventCollector;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let relays = vec!["wss://relay.damus.io".to_string()];
    let collector = Arc::new(EventCollector::new(relays).await?);

    let app = create_app(collector);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("SentryStr API running on http://localhost:3000");
    axum::serve(listener, app).await?;

    Ok(())
}
```

## 📱 Direct Message Alerts

Get instant notifications for critical errors:

### With Core Client

```rust
use sentrystr_core::{Config, DirectMessageBuilder, NostrSentryClient, Level};
use nostr::prelude::*;
use nostr_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup main client
    let keys = Keys::generate();
    let config = Config::new(keys.secret_key().display_secret().to_string(), vec!["wss://relay.damus.io".to_string()]);
    let mut client = NostrSentryClient::new(config).await?;

    // Setup DM alerts
    let dm_keys = Keys::generate();
    let dm_client = Client::new(dm_keys.clone());
    dm_client.add_relay("wss://relay.damus.io").await?;
    dm_client.connect().await;

    let recipient = PublicKey::from_bech32("npub1...")?;
    let dm_sender = DirectMessageBuilder::new()
        .with_client(dm_client)
        .with_keys(dm_keys)
        .with_recipient(recipient)
        .with_min_level(Level::Error)
        .with_nip17(true)  // Use NIP-17 encryption
        .build()?;

    client.set_direct_messaging(dm_sender);

    // This will send both to relays AND as a DM
    client.capture_error("Critical system failure").await?;

    Ok(())
}
```

### With Tracing

```rust
use sentrystr_tracing::{SentryStrTracingBuilder, builder::DirectMessageConfig};
use sentrystr_core::Level;
use nostr::PublicKey;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let relays = vec!["wss://relay.damus.io".to_string()];
    let recipient = PublicKey::from_bech32("npub1...")?;

    let dm_config = DirectMessageConfig::new(recipient, relays.clone())
        .with_min_level(Level::Error)
        .with_nip17(true);

    SentryStrTracingBuilder::new()
        .with_generated_keys_and_relays(relays)
        .with_direct_messaging(dm_config)
        .init()
        .await?;

    info!("This logs normally");
    error!("This sends a DM too!"); // Triggers direct message

    Ok(())
}
```

## 🏗️ Architecture

### Crates

- **`sentrystr-core`** - Core client for sending events
- **`sentrystr-tracing`** - Tracing integration layer
- **`sentrystr-collector`** - Event collection and monitoring
- **`sentrystr-api`** - REST API server

### Features

- 🔄 **Decentralized** - Uses Nostr relays, no central server
- 🔐 **Encrypted** - Supports NIP-44 and NIP-17 encryption
- 📱 **Real-time Alerts** - Direct message notifications
- 🏷️ **Structured** - Rich event data with custom fields
- 🚀 **Fast** - Async, non-blocking operations
- 🔌 **Pluggable** - Works with existing tracing setups

## 📝 License

MIT License - see LICENSE file for details.


---

**SentryStr** - Decentralized error tracking for the Nostr age 🚀