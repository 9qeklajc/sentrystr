# SentryStr Collector

Event collection and monitoring service for the SentryStr ecosystem.

## Overview

The SentryStr Collector provides tools for collecting, filtering, and monitoring error events from Nostr relays. It can be used to build monitoring dashboards, alerting systems, and event analysis tools.

## Features

- **Real-time Event Collection**: Subscribe to live events from multiple Nostr relays
- **Event Filtering**: Filter events by author, level, time range, and custom criteria
- **Private Message Alerts**: Send DM notifications for critical events
- **Batch Collection**: Collect historical events with flexible queries
- **Multiple Relay Support**: Connect to multiple Nostr relays simultaneously

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
sentrystr-collector = "0.1.0"
```

Basic usage:

```rust
use sentrystr_collector::{EventCollector, EventFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let relays = vec!["wss://relay.damus.io".to_string()];
    let collector = EventCollector::new(relays).await?;

    // Collect recent events
    let filter = EventFilter::new().with_limit(10);
    let events = collector.collect_events(filter).await?;

    for event in events {
        println!("Event: {:?} from {}", event.event.message, event.author);
    }

    Ok(())
}
```

## Event Filtering

Filter events by various criteria:

```rust
use sentrystr_collector::{EventCollector, EventFilter};
use sentrystr::Level;
use nostr::Keys;
use chrono::{Utc, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let relays = vec!["wss://relay.damus.io".to_string()];
    let collector = EventCollector::new(relays).await?;

    let author = Keys::generate().public_key();
    let since = Utc::now() - Duration::hours(24);

    let filter = EventFilter::new()
        .with_author(author)
        .with_level(Level::Error)
        .with_since(since)
        .with_limit(50);

    let events = collector.collect_events(filter).await?;
    println!("Found {} error events in the last 24 hours", events.len());

    Ok(())
}
```

## Real-time Monitoring

Subscribe to live events:

```rust
use sentrystr_collector::{EventCollector, EventFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let relays = vec!["wss://relay.damus.io".to_string()];
    let collector = EventCollector::new(relays).await?;

    let mut rx = collector.subscribe_to_events(EventFilter::new()).await?;

    while let Some(event) = rx.recv().await {
        println!("Live event: {:?}", event.event.message);

        // Handle the event (store, alert, etc.)
    }

    Ok(())
}
```

## Private Message Alerts

Set up DM alerts for specific event types:

```rust
use sentrystr_collector::{EventCollector, PrivateMessageConfig};
use sentrystr::Level;
use nostr::Keys;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let relays = vec!["wss://relay.damus.io".to_string()];
    let mut collector = EventCollector::new(relays).await?;

    // Setup DM alerts for errors
    let recipient = Keys::generate().public_key();
    let dm_config = PrivateMessageConfig {
        recipient_pubkey: recipient,
        min_level: Some(Level::Error),
        use_nip17: true,
    };

    collector = collector.with_private_messaging(dm_config)?;

    // Now critical events will also send DMs
    let mut rx = collector.subscribe_to_events(EventFilter::new()).await?;
    while let Some(event) = rx.recv().await {
        println!("Event received and processed for DM alerts");
    }

    Ok(())
}
```

## Integration

This crate works seamlessly with other SentryStr ecosystem crates:

- **[sentrystr](https://crates.io/crates/sentrystr)**: Core library for publishing events
- **[sentrystr-api](https://crates.io/crates/sentrystr-api)**: REST API for serving collected events
- **[sentrystr-tracing](https://crates.io/crates/sentrystr-tracing)**: Tracing integration

## License

MIT License - see [LICENSE](LICENSE) file for details.