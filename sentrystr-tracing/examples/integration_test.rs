use nostr::prelude::*;
use sentrystr::Level;
use sentrystr_tracing::{builder::DirectMessageConfig, SentryStrTracingBuilder};
use tracing::{debug, error, info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let relays = vec![
        "wss://relay.damus.io".to_string(),
        "wss://nos.lol".to_string(),
        "wss://nostr.chaima.info".to_string(),
        "wss://relay.nostr.info".to_string(),
    ];

    let recipient_npub = "npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps";
    let recipient_pubkey = PublicKey::from_bech32(recipient_npub)?;

    let dm_config = DirectMessageConfig::new(recipient_pubkey, relays.clone())
        .with_min_level(Level::Error)
        .with_nip17(true);

    SentryStrTracingBuilder::new()
        .with_generated_keys_and_relays(relays)
        .with_direct_messaging(dm_config)
        .with_min_level(tracing::Level::DEBUG)
        .init_with_env_filter("info")
        .await?;

    println!("🧪 Starting SentryStr Tracing Integration Test");

    info!(
        test_id = "integration_001",
        component = "tracing_test",
        "Integration test started"
    );

    debug!("This debug message should not trigger a DM");

    info!(
        user_count = 150,
        active_sessions = 23,
        "System stats updated"
    );

    warn!(
        disk_usage = 85.5,
        threshold = 80.0,
        partition = "/var/log",
        "Disk usage warning - no DM expected"
    );

    error!(
        error_code = "DB_CONNECTION_FAILED",
        database = "user_db",
        retry_attempt = 1,
        timeout_ms = 5000,
        "Database connection error - DM should be sent"
    );

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    error!(
        severity = "critical",
        service = "payment_processor",
        affected_transactions = 47,
        estimated_loss = 12500.50,
        "Critical payment processing failure - DM should be sent"
    );

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    info!(
        test_id = "integration_001",
        status = "completed",
        events_sent = 5,
        dm_events = 2,
        "Integration test completed successfully"
    );

    println!("✅ Integration test completed");
    println!("📱 Check your Nostr client for direct messages");
    println!("🔍 Expected DMs: 2 (for the error events only)");

    Ok(())
}
