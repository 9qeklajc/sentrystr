use std::str::FromStr;

use crate::{Config, DirectMessageBuilder, Event, Level, NostrSentryClient, Result};
use nostr::prelude::*;
use nostr_sdk::prelude::*;

/// Example demonstrating how to use both event capture and direct messaging simultaneously
pub async fn run_combined_example() -> Result<()> {
    // Setup configuration for the main client
    let keys = Keys::generate();
    let config = Config::new(
        keys.secret_key().display_secret().to_string(),
        vec![
            "wss://relay.damus.io".to_string(),
            "wss://nos.lol".to_string(),
            "wss://nostr.chaima.info".to_string(),
        ],
    );

    // Create the main NostrSentryClient
    let mut client = NostrSentryClient::new(config).await?;

    // Get the client internals to create a DirectMessageSender
    // This would typically be done through a builder pattern or configuration
    let keys = Keys::generate(); // In practice, you'd use existing keys
    let nostr_client = Client::new(keys.clone());

    // Add relays to the DM client
    nostr_client.add_relay("wss://relay.damus.io").await?;
    nostr_client.add_relay("wss://nos.lol").await?;
    nostr_client.add_relay("wss://nostr.chaima.info").await?;
    nostr_client.connect().await;

    // Create recipient public key - npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps
    let recipient_pubkey =
        PublicKey::from_str("npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps")
            .map_err(|e| crate::SentryStrError::Config(format!("Invalid pubkey: {}", e)))?;

    // Build the DirectMessageSender with configuration
    let dm_sender = DirectMessageBuilder::new()
        .with_client(nostr_client)
        .with_keys(keys)
        .with_recipient(recipient_pubkey)
        .with_min_level(Level::Warning) // Only send DMs for warnings and above
        .with_nip17(true) // Use NIP-17 for better privacy
        .build()?;

    // Configure the client to use direct messaging
    client.set_direct_messaging(dm_sender);

    // Now both functionalities work together:

    // 1. Capture an info event - this will be logged but no DM sent (below min level)
    let info_event = Event::new()
        .with_message("Application started successfully")
        .with_level(Level::Info);

    let event_id1 = client.capture_event(info_event).await?;
    println!("Info event captured: {}", event_id1);

    // 2. Capture a warning event - this will be logged AND a DM will be sent
    let warning_event = Event::new()
        .with_message("High memory usage detected")
        .with_level(Level::Warning);

    let event_id2 = client.capture_event(warning_event).await?;
    println!("Warning event captured with DM: {}", event_id2);

    // 3. Send a standalone direct message
    client
        .send_direct_message("Manual alert: System maintenance required")
        .await?;
    println!("Standalone DM sent");

    // 4. Capture an error - this will also trigger a DM
    let error_event = Event::new()
        .with_message("hello world")
        .with_level(Level::Error);

    let event_id3 = client.capture_event(error_event).await?;
    println!("Error event captured with DM: {}", event_id3);

    client.disconnect().await?;
    Ok(())
}

/// Example of creating a client with direct messaging using the builder pattern
pub async fn create_client_with_dm_builder() -> Result<NostrSentryClient> {
    let keys = Keys::generate();
    let config = Config::new(
        keys.secret_key().display_secret().to_string(),
        vec!["wss://relay.damus.io".to_string()],
    );

    let keys = Keys::generate();
    let nostr_client = Client::new(keys.clone());
    nostr_client.add_relay("wss://relay.damus.io").await?;
    nostr_client.connect().await;

    let recipient_pubkey =
        PublicKey::from_str("npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps")
            .map_err(|e| crate::SentryStrError::Config(format!("Invalid pubkey: {}", e)))?;

    let dm_sender = DirectMessageBuilder::new()
        .with_client(nostr_client)
        .with_keys(keys)
        .with_recipient(recipient_pubkey)
        .with_min_level(Level::Error)
        .with_nip17(false) // Use NIP-44 instead
        .build()?;

    let client = NostrSentryClient::new(config)
        .await?
        .with_direct_messaging(dm_sender);

    Ok(client)
}

/// Example of switching between different DM configurations
pub async fn switch_dm_configurations(client: &mut NostrSentryClient) -> Result<()> {
    let keys = Keys::generate();
    let nostr_client = Client::new(keys.clone());
    nostr_client.add_relay("wss://relay.damus.io").await?;
    nostr_client.connect().await;

    // First configuration: NIP-17 for all events
    let recipient1 =
        PublicKey::from_str("npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps")
            .map_err(|e| crate::SentryStrError::Config(format!("Invalid pubkey: {}", e)))?;

    let dm_sender1 = DirectMessageBuilder::new()
        .with_client(nostr_client.clone())
        .with_keys(keys.clone())
        .with_recipient(recipient1)
        .with_nip17(true)
        .build()?;

    client.set_direct_messaging(dm_sender1);

    // Test with first configuration
    client.capture_error("Error with first DM config").await?;

    // Switch to second configuration: NIP-44 for errors only
    let recipient2 =
        PublicKey::from_str("npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps")
            .map_err(|e| crate::SentryStrError::Config(format!("Invalid pubkey: {}", e)))?;

    let dm_sender2 = DirectMessageBuilder::new()
        .with_client(nostr_client)
        .with_keys(keys)
        .with_recipient(recipient2)
        .with_min_level(Level::Error)
        .with_nip17(false)
        .build()?;

    client.set_direct_messaging(dm_sender2);

    // Test with second configuration
    client.capture_error("Error with second DM config").await?;

    // Disable direct messaging
    client.remove_direct_messaging();

    // This won't send a DM
    client.capture_error("Error without DM").await?;

    Ok(())
}
