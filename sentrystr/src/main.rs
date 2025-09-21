use nostr::Keys;
use sentrystr::{Config, Event, Level, NostrSentryClient};

#[tokio::main]
async fn main() -> sentrystr::Result<()> {
    let secret_key = Keys::generate().secret_key().display_secret().to_string();
    let relays = vec![
        "wss://relay.damus.io".to_string(),
        "wss://nostr.chaima.info".to_string(),
    ];

    let config = Config::new(secret_key, relays);
    let client = NostrSentryClient::new(config).await?;

    let event = Event::new()
        .with_message("Test message from SentryStr Core")
        .with_level(Level::Info)
        .with_tag("component", "main")
        .with_extra("version", serde_json::json!("0.1.0"));

    let event_id = client.capture_event(event).await?;
    println!("Published event: {}", event_id);

    client.capture_message("Simple message test").await?;
    client.capture_error("Error message test").await?;

    client.disconnect().await?;
    Ok(())
}
