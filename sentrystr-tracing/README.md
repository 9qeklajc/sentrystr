# SentryStr Tracing

A tracing integration for SentryStr that provides seamless logging and alerting through Nostr relays with optional direct messaging capabilities.

## Features

- **Custom Tracing Layer**: Integrates with the `tracing` ecosystem
- **Structured Logging**: Captures all tracing fields and metadata
- **Direct Messaging**: Optional DM alerts for critical events
- **Level Filtering**: Configurable event level thresholds
- **NIP-17/NIP-44 Support**: Choose between gift wrapping or direct encryption
- **Builder Pattern**: Clean, fluent API for configuration

## Quick Start

### Basic Usage

```rust
use sentrystr_tracing::SentryStrTracingBuilder;
use tracing::{info, warn, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let relays = vec!["wss://relay.damus.io".to_string()];

    SentryStrTracingBuilder::new()
        .with_generated_keys_and_relays(relays)
        .with_min_level(tracing::Level::INFO)
        .init()
        .await?;

    info!("Application started");
    warn!(cpu_usage = 85.5, "High CPU usage");
    error!(error_code = 500, "Database connection failed");

    Ok(())
}
```

### With Direct Messaging

```rust
use nostr::prelude::*;
use sentrystr_core::Level;
use sentrystr_tracing::{builder::DirectMessageConfig, SentryStrTracingBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let relays = vec!["wss://relay.damus.io".to_string()];
    let recipient_pubkey = PublicKey::from_bech32("npub1...")?;

    let dm_config = DirectMessageConfig::new(recipient_pubkey, relays.clone())
        .with_min_level(Level::Error)
        .with_nip17(true);

    SentryStrTracingBuilder::new()
        .with_generated_keys_and_relays(relays)
        .with_direct_messaging(dm_config)
        .init()
        .await?;

    // This will send both to Nostr and as a DM
    error!("Critical system failure");

    Ok(())
}
```

## Configuration Options

### Builder Methods

- `with_config(config)` - Use existing SentryStr config
- `with_generated_keys_and_relays(relays)` - Generate new keys
- `with_secret_key_and_relays(key, relays)` - Use specific key
- `with_direct_messaging(dm_config)` - Enable DM alerts
- `with_min_level(level)` - Set minimum tracing level
- `with_fields(include)` - Include/exclude custom fields
- `with_metadata(include)` - Include/exclude tracing metadata

### Direct Message Configuration

```rust
let dm_config = DirectMessageConfig::new(recipient_pubkey, relays)
    .with_min_level(Level::Warning)  // Only warn+ events
    .with_nip17(true);               // Use NIP-17 encryption
```

## Examples

Run the examples to see the integration in action:

```bash
# Basic tracing without DMs
cargo run --example basic_usage

# With direct message alerts
cargo run --example with_direct_messages

# Complex structured logging
cargo run --example custom_fields

# Integration test
cargo run --example integration_test
```

## Integration Patterns

### Web Application

```rust
use axum::{routing::get, Router};
use sentrystr_tracing::SentryStrTracingBuilder;
use tracing::{info, error};

#[tokio::main]
async fn main() {
    SentryStrTracingBuilder::new()
        .with_generated_keys_and_relays(vec!["wss://relay.damus.io".to_string()])
        .init_with_env_filter("info,axum=debug")
        .await
        .expect("Failed to initialize tracing");

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    info!("Starting web server on port 3000");
    // Server will automatically log requests through tracing
}
```

### Background Service

```rust
use sentrystr_tracing::SentryStrTracingBuilder;
use tracing::{info, warn, error, instrument};

#[instrument]
async fn process_job(job_id: u64) -> Result<(), Box<dyn std::error::Error>> {
    info!(job_id = job_id, "Processing job");

    // Simulate work
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    if job_id % 10 == 0 {
        error!(job_id = job_id, "Job processing failed");
        return Err("Processing failed".into());
    }

    info!(job_id = job_id, "Job completed successfully");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    SentryStrTracingBuilder::new()
        .with_generated_keys_and_relays(vec!["wss://relay.damus.io".to_string()])
        .with_min_level(tracing::Level::INFO)
        .init()
        .await?;

    for job_id in 1..=20 {
        if let Err(e) = process_job(job_id).await {
            warn!(job_id = job_id, error = %e, "Job failed, will retry");
        }
    }

    Ok(())
}
```

## Advanced Usage

### Custom Event Processing

The layer captures all structured fields from your tracing events:

```rust
error!(
    user_id = 12345,
    error_type = "authentication_failure",
    ip_address = "192.168.1.100",
    user_agent = "Mozilla/5.0...",
    "User authentication failed"
);
```

All fields are preserved in the SentryStr event and available for filtering and alerting.

### Environment-based Configuration

```rust
use std::env;

let dm_enabled = env::var("SENTRYSTR_DM_ENABLED").unwrap_or_default() == "true";
let min_level = env::var("SENTRYSTR_MIN_LEVEL")
    .unwrap_or_else(|_| "info".to_string());

let mut builder = SentryStrTracingBuilder::new()
    .with_generated_keys_and_relays(relays);

if dm_enabled {
    let recipient = env::var("SENTRYSTR_DM_RECIPIENT")?;
    let pubkey = PublicKey::from_bech32(&recipient)?;
    builder = builder.with_dm_recipient(pubkey, relays.clone());
}

builder.init().await?;
```

## Best Practices

1. **Use appropriate log levels** - Don't spam with debug messages
2. **Structure your logs** - Include relevant context fields
3. **Set DM thresholds carefully** - Only alert on actionable events
4. **Use spans for request tracing** - Leverage `#[instrument]`
5. **Configure relay redundancy** - Use multiple relays for reliability

## Performance Considerations

- Events are processed asynchronously to avoid blocking
- DM sending is optional and non-blocking
- Failed Nostr sends are logged but don't crash the application
- Configurable field inclusion to control overhead

## Error Handling

The layer handles errors gracefully:

- Failed Nostr events are logged to stderr
- DM failures don't affect event capture
- Network issues are retried automatically
- Malformed events are skipped with warnings