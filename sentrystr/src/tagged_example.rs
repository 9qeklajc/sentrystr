use nostr::Keys;
use sentrystr::{Config, Event, Level, NostrSentryClient};

#[tokio::main]
async fn main() -> sentrystr::Result<()> {
    println!("SentryStr Tagged Events Example");
    println!("==============================");

    let sender_keys = Keys::generate();
    println!("Sender public key: {}", sender_keys.public_key());

    let relays = vec![
        "wss://relay.damus.io".to_string(),
        "wss://nostr.chaima.info".to_string(),
    ];

    let config = Config::new(
        sender_keys.secret_key().display_secret().to_string(),
        relays,
    );

    let client = NostrSentryClient::new(config).await?;

    println!();
    println!("Publishing events with different tags...");

    let auth_error = Event::new()
        .with_message("Authentication failed for user admin")
        .with_level(Level::Error)
        .with_service_tag("auth-service")
        .with_environment_tag("production")
        .with_component_tag("login")
        .with_severity_tag(&Level::Error)
        .with_tag("user_id", "admin")
        .with_extra("attempt_count", serde_json::json!(3));

    let event_id = client.capture_event(auth_error).await?;
    println!("Published auth error event: {}", event_id);

    let db_warning = Event::new()
        .with_message("Database connection pool running low")
        .with_level(Level::Warning)
        .with_service_tag("database-service")
        .with_environment_tag("production")
        .with_component_tag("connection-pool")
        .with_severity_tag(&Level::Warning)
        .with_extra("pool_size", serde_json::json!(2));

    let event_id = client.capture_event(db_warning).await?;
    println!("Published database warning event: {}", event_id);

    let api_info = Event::new()
        .with_message("API request processed successfully")
        .with_level(Level::Info)
        .with_service_tag("api-gateway")
        .with_environment_tag("production")
        .with_component_tag("request-handler")
        .with_severity_tag(&Level::Info)
        .with_tag("endpoint", "/users")
        .with_tag("method", "GET")
        .with_extra("response_time_ms", serde_json::json!(45));

    let event_id = client.capture_event(api_info).await?;
    println!("Published API info event: {}", event_id);

    let test_debug = Event::new()
        .with_message("Debug trace for test environment")
        .with_level(Level::Debug)
        .with_service_tag("test-runner")
        .with_environment_tag("staging")
        .with_component_tag("unit-tests")
        .with_severity_tag(&Level::Debug)
        .with_tag("test_name", "auth_test_001");

    let event_id = client.capture_event(test_debug).await?;
    println!("Published test debug event: {}", event_id);

    println!();
    println!("All events published successfully!");
    println!();
    println!("To collect these events with filtering, try:");
    println!(
        "cargo run --bin sentrystr-collector -- collect --author {}",
        sender_keys.public_key()
    );
    println!(
        "cargo run --bin sentrystr-collector -- collect --author {} --service auth-service",
        sender_keys.public_key()
    );
    println!(
        "cargo run --bin sentrystr-collector -- collect --author {} --environment production",
        sender_keys.public_key()
    );
    println!(
        "cargo run --bin sentrystr-collector -- collect --author {} --component login",
        sender_keys.public_key()
    );
    println!(
        "cargo run --bin sentrystr-collector -- collect --author {} --level error",
        sender_keys.public_key()
    );
    println!(
        "cargo run --bin sentrystr-collector -- collect --author {} --tag user_id=admin",
        sender_keys.public_key()
    );

    client.disconnect().await?;
    Ok(())
}
