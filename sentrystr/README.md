# SentryStr

A decentralized error tracking and alerting system using the Nostr protocol.

## Overview

SentryStr provides a Rust library for publishing structured error events and logs to the Nostr network, enabling decentralized monitoring and alerting. It offers a familiar API similar to traditional error tracking services like Sentry, but leverages the censorship-resistant and decentralized nature of Nostr.

## Features

- **Decentralized Logging**: Publish error events to multiple Nostr relays
- **Structured Events**: Rich event data with levels, timestamps, and custom fields
- **Direct Message Alerts**: Optional encrypted DM notifications for critical errors
- **Multiple Event Types**: Support for errors, warnings, info messages, and custom events
- **Encryption Support**: NIP-44 and NIP-59 encryption for sensitive data
- **Flexible Configuration**: Easy setup with sensible defaults

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
sentrystr = "0.1.0"
```

Basic usage:

```rust
use sentrystr::{Config, Event, Level, NostrSentryClient};
use nostr::Keys;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup
    let keys = Keys::generate();
    let relays = vec!["wss://relay.damus.io".to_string()];
    let config = Config::new(keys.secret_key().display_secret().to_string(), relays);

    // Create client
    let client = NostrSentryClient::new(config).await?;

    // Send events
    let event = Event::new()
        .with_message("Something went wrong")
        .with_level(Level::Error);

    client.capture_event(event).await?;
    client.capture_error("Database connection failed").await?;
    client.capture_message("System started").await?;

    Ok(())
}
```

## Event Levels

- `Debug`: Detailed diagnostic information
- `Info`: General informational messages
- `Warning`: Warning messages for potentially harmful situations
- `Error`: Error events that might still allow the application to continue
- `Fatal`: Very severe error events that might cause the application to abort

## Advanced Usage

### Direct Message Alerts

Set up encrypted direct message notifications for critical errors:

```rust
use sentrystr::{Config, DirectMessageBuilder, NostrSentryClient};
use nostr::prelude::*;
use nostr_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup main client
    let keys = Keys::generate();
    let relays = vec!["wss://relay.damus.io".to_string()];
    let config = Config::new(keys.secret_key().display_secret().to_string(), relays.clone());
    let mut client = NostrSentryClient::new(config).await?;

    // Setup direct messaging
    let dm_keys = Keys::generate();
    let dm_client = Client::new(dm_keys.clone());
    dm_client.add_relay("wss://relay.damus.io").await?;
    dm_client.connect().await;

    let recipient = Keys::generate().public_key();
    let dm_sender = DirectMessageBuilder::new()
        .with_client(dm_client)
        .with_keys(dm_keys)
        .with_recipient(recipient)
        .with_min_level(Level::Error)
        .with_nip17(true)
        .build()?;

    client.set_direct_messaging(dm_sender);

    // Now errors will also send DMs
    client.capture_error("Critical system failure").await?;

    Ok(())
}
```

### Custom Event Fields

Add custom fields to your events:

```rust
use sentrystr::{Event, Level};
use serde_json::json;

let event = Event::new()
    .with_message("User authentication failed")
    .with_level(Level::Warning)
    .with_field("user_id", json!("12345"))
    .with_field("ip_address", json!("192.168.1.1"))
    .with_field("user_agent", json!("Mozilla/5.0..."));
```

## Integration with Other Crates

- **[sentrystr-tracing](https://crates.io/crates/sentrystr-tracing)**: Integration with the `tracing` ecosystem
- **[sentrystr-collector](https://crates.io/crates/sentrystr-collector)**: Event collection and monitoring tools
- **[sentrystr-api](https://crates.io/crates/sentrystr-api)**: REST API for querying events

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.