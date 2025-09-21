# SentryStr API

REST API server for the SentryStr ecosystem, providing HTTP endpoints for querying and serving error events.

## Overview

The SentryStr API provides a REST interface for accessing SentryStr events collected from Nostr relays. It's designed to be used with monitoring dashboards, alerting systems, and other tools that need HTTP access to event data.

## Features

- **RESTful API**: Clean HTTP endpoints for event querying
- **Event Filtering**: Query events by author, level, time range, and more
- **JSON Responses**: Structured JSON responses for easy integration
- **CORS Support**: Cross-origin resource sharing for web applications
- **Health Checks**: Built-in health check endpoint
- **Async Performance**: Built on Axum for high-performance async handling

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
sentrystr-api = "0.1.0"
```

Basic server setup:

```rust
use sentrystr_api::create_app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("SentryStr API server running on http://localhost:3000");

    axum::serve(listener, app).await?;
    Ok(())
}
```

## API Endpoints

### GET /health

Health check endpoint that returns the server status.

**Response:**
```json
{
  "status": "ok",
  "timestamp": "2024-01-01T00:00:00Z"
}
```

**Example:**
```bash
curl "http://localhost:3000/health"
```

### GET /events

Query events with optional filters.

**Query Parameters:**
- `limit`: Maximum number of events to return (default: 50, max: 1000)
- `level`: Filter by event level (`debug`, `info`, `warning`, `error`, `fatal`)
- `author`: Filter by author's public key (hex or npub format)
- `since`: ISO 8601 timestamp to filter events since
- `until`: ISO 8601 timestamp to filter events until

**Response:**
```json
{
  "events": [
    {
      "id": "event-uuid",
      "message": "Error message",
      "level": "error",
      "timestamp": "2024-01-01T00:00:00Z",
      "author": "npub1...",
      "fields": {
        "custom_field": "value"
      }
    }
  ],
  "count": 1,
  "has_more": false
}
```

**Examples:**
```bash
# Get last 10 events
curl "http://localhost:3000/events?limit=10"

# Get error events only
curl "http://localhost:3000/events?level=error"

# Get events from specific author
curl "http://localhost:3000/events?author=npub1..."

# Get events from last 24 hours
curl "http://localhost:3000/events?since=2024-01-01T00:00:00Z"

# Combined filters
curl "http://localhost:3000/events?level=error&limit=50&since=2024-01-01T00:00:00Z"
```

### GET /events/{author}

Get events from a specific author.

**Path Parameters:**
- `author`: Author's public key in hex or npub format

**Query Parameters:**
Same as `/events` endpoint except `author` is not needed.

**Example:**
```bash
curl "http://localhost:3000/events/npub1.../events?limit=20"
```

## Configuration

The API server can be configured with environment variables:

- `SENTRYSTR_API_PORT`: Server port (default: 3000)
- `SENTRYSTR_API_HOST`: Server host (default: 0.0.0.0)
- `SENTRYSTR_RELAYS`: Comma-separated list of Nostr relays to connect to

## Integration with SentryStr Collector

The API typically works with the SentryStr Collector to provide event data:

```rust
use sentrystr_api::create_app;
use sentrystr_collector::EventCollector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start background event collection
    let relays = vec!["wss://relay.damus.io".to_string()];
    let collector = EventCollector::new(relays).await?;

    // Start background collection task
    tokio::spawn(async move {
        // Collection logic here
    });

    // Start API server
    let app = create_app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("SentryStr API server running on http://localhost:3000");
    axum::serve(listener, app).await?;

    Ok(())
}
```

## Docker Support

A Dockerfile is provided for containerized deployment:

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin sentrystr-api

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/sentrystr-api /usr/local/bin/sentrystr-api
EXPOSE 3000
CMD ["sentrystr-api"]
```

## Related Crates

- **[sentrystr](https://crates.io/crates/sentrystr)**: Core library for publishing events
- **[sentrystr-collector](https://crates.io/crates/sentrystr-collector)**: Event collection from Nostr relays
- **[sentrystr-tracing](https://crates.io/crates/sentrystr-tracing)**: Tracing integration

## License

MIT License - see [LICENSE](LICENSE) file for details.