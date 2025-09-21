use nostr::prelude::*;
use sentrystr::Level;
use sentrystr_tracing::{builder::DirectMessageConfig, SentryStrTracingBuilder};
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let relays = vec![
        "wss://relay.damus.io".to_string(),
        "wss://nos.lol".to_string(),
    ];

    let recipient_npub = "npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps";
    let recipient_pubkey = PublicKey::from_bech32(recipient_npub)?;

    let dm_config = DirectMessageConfig::new(recipient_pubkey, relays.clone())
        .with_min_level(Level::Warning)
        .with_nip17(true);

    SentryStrTracingBuilder::new()
        .with_generated_keys_and_relays(relays)
        .with_direct_messaging(dm_config)
        .with_min_level(tracing::Level::INFO)
        .init()
        .await?;

    info!("Application started with DM notifications");
    info!(version = "1.0.0", "System initialized");

    warn!(
        cpu_usage = 90.5,
        memory_usage = 85.2,
        "High resource usage detected"
    );

    error!(
        error_type = "connection_failure",
        retry_count = 3,
        "Database connection failed after retries"
    );

    error!(
        severity = "critical",
        affected_users = 1500,
        "Payment processing system failure"
    );

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    info!("Application shutdown complete");

    Ok(())
}
