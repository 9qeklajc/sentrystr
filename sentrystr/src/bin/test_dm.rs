use nostr::prelude::*;
use nostr_sdk::prelude::*;
use sentrystr::{Config, DirectMessageBuilder, Event, Level, NostrSentryClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Convert npub to hex pubkey
    let npub = "npub18kpn83drge7x9vz4cuhh7xta79sl4tfq55se4e554yj90s8y3f7qa49nps";
    let recipient_pubkey = PublicKey::from_bech32(npub)
        .map_err(|e| sentrystr::SentryStrError::Config(format!("Invalid npub: {}", e)))?;

    println!("Recipient pubkey: {}", recipient_pubkey);

    // Setup configuration for the main client
    let keys = Keys::generate();
    println!("Sender pubkey: {}", keys.public_key());

    let config = Config::new(
        keys.secret_key().display_secret().to_string(),
        vec![
            "wss://relay.damus.io".to_string(),
            "wss://nos.lol".to_string(),
            "wss://relay.nostr.info".to_string(),
        ],
    );

    // Create the main NostrSentryClient
    let mut client = NostrSentryClient::new(config).await?;
    println!("✅ Main client connected");

    // Create DM client with same keys for simplicity
    let dm_client = Client::new(keys.clone());
    dm_client.add_relay("wss://relay.damus.io").await?;
    dm_client.add_relay("wss://nos.lol").await?;
    dm_client.add_relay("wss://relay.nostr.info").await?;
    dm_client.connect().await;
    println!("✅ DM client connected");

    // Test 1: NIP-17 messaging
    println!("\n🧪 Test 1: NIP-17 Direct Messaging");
    let dm_sender_nip17 = DirectMessageBuilder::new()
        .with_client(dm_client.clone())
        .with_keys(keys.clone())
        .with_recipient(recipient_pubkey)
        .with_min_level(Level::Info)
        .with_nip17(true) // Use NIP-17
        .build()?;

    client.set_direct_messaging(dm_sender_nip17);

    // Send a test event that should trigger a DM
    let test_event = Event::new()
        .with_message("Test message from SentryStr - NIP-17")
        .with_level(Level::Warning)
        .with_tag("test", "nip17");

    let event_id = client.capture_event(test_event).await?;
    println!("📨 Sent event with NIP-17 DM: {}", event_id);

    // Send standalone DM
    client
        .send_direct_message("Standalone NIP-17 test message from SentryStr")
        .await?;
    println!("📨 Sent standalone NIP-17 DM");

    // Wait a bit for messages to propagate
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Test 2: NIP-44 messaging
    println!("\n🧪 Test 2: NIP-44 Direct Messaging");
    let dm_sender_nip44 = DirectMessageBuilder::new()
        .with_client(dm_client.clone())
        .with_keys(keys.clone())
        .with_recipient(recipient_pubkey)
        .with_min_level(Level::Error) // Only errors and above
        .with_nip17(false) // Use NIP-44
        .build()?;

    client.set_direct_messaging(dm_sender_nip44);

    // This should NOT trigger a DM (level too low)
    let info_event = Event::new()
        .with_message("Info message - should not send DM")
        .with_level(Level::Info);

    let info_id = client.capture_event(info_event).await?;
    println!("📝 Sent info event (no DM expected): {}", info_id);

    // This SHOULD trigger a DM
    let error_event = Event::new()
        .with_message("Error message from SentryStr - NIP-44")
        .with_level(Level::Error)
        .with_tag("test", "nip44")
        .with_extra("error_code", serde_json::json!(500));

    let error_id = client.capture_event(error_event).await?;
    println!("📨 Sent error event with NIP-44 DM: {}", error_id);

    // Send another standalone DM
    client
        .send_direct_message("Standalone NIP-44 test message from SentryStr")
        .await?;
    println!("📨 Sent standalone NIP-44 DM");

    // Wait for messages to propagate
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Test 3: Level filtering test
    println!("\n🧪 Test 3: Level Filtering");
    let dm_sender_warning = DirectMessageBuilder::new()
        .with_client(dm_client)
        .with_keys(keys.clone())
        .with_recipient(recipient_pubkey)
        .with_min_level(Level::Warning) // Warning and above
        .with_nip17(true)
        .build()?;

    client.set_direct_messaging(dm_sender_warning);

    // These should NOT trigger DMs
    client.capture_message("Debug message").await?;
    println!("📝 Sent debug message (no DM expected)");

    let info_event2 = Event::new()
        .with_message("Info message 2")
        .with_level(Level::Info);
    client.capture_event(info_event2).await?;
    println!("📝 Sent info message (no DM expected)");

    // These SHOULD trigger DMs
    let warning_event = Event::new()
        .with_message("Warning: High CPU usage detected")
        .with_level(Level::Warning);
    client.capture_event(warning_event).await?;
    println!("📨 Sent warning event with DM");

    client
        .capture_error("Critical database connection error")
        .await?;
    println!("📨 Sent error event with DM");

    let fatal_event = Event::new()
        .with_message("Fatal: System crash imminent")
        .with_level(Level::Fatal);
    client.capture_event(fatal_event).await?;
    println!("📨 Sent fatal event with DM");

    // Wait for final messages
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Test 4: Disable and re-enable DMs
    println!("\n🧪 Test 4: Dynamic Configuration");

    // Disable DMs
    client.remove_direct_messaging();
    client
        .capture_error("This error should NOT send a DM")
        .await?;
    println!("📝 Sent error with DMs disabled (no DM expected)");

    // Re-enable with different config
    let final_client = Client::new(keys.clone());
    final_client.add_relay("wss://relay.damus.io").await?;
    final_client.add_relay("wss://nos.lol").await?;
    final_client.connect().await;

    let dm_sender_final = DirectMessageBuilder::new()
        .with_client(final_client)
        .with_keys(keys.clone())
        .with_recipient(recipient_pubkey)
        .with_nip17(true)
        .build()?;

    client.set_direct_messaging(dm_sender_final);
    client
        .send_direct_message("Final test message - DMs re-enabled!")
        .await?;
    println!("📨 Sent final test DM");

    println!("\n✅ All tests completed!");
    println!("📱 Check your Nostr client for the direct messages");
    println!("🔑 Sender npub: {}", keys.public_key().to_bech32().unwrap());

    client.disconnect().await?;
    Ok(())
}
