use sentrystr_tracing::SentryStrTracingBuilder;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let relays = vec![
        "wss://relay.damus.io".to_string(),
        "wss://nos.lol".to_string(),
    ];

    SentryStrTracingBuilder::new()
        .with_generated_keys_and_relays(relays)
        .with_min_level(tracing::Level::INFO)
        .init()
        .await?;

    info!("Application started successfully");
    info!(user_id = 12345, action = "login", "User logged in");

    warn!(
        temperature = 85.5,
        threshold = 80.0,
        "System temperature is above threshold"
    );

    error!(
        error_code = 500,
        component = "database",
        "Failed to connect to database"
    );

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    info!("Application shutting down");

    Ok(())
}
