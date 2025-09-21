use nostr::Keys;
use sentrystr::{Config, EncryptionHelper, Event, Level, NostrSentryClient};

#[tokio::main]
async fn main() -> sentrystr::Result<()> {
    println!("SentryStr NIP-44 Encryption Example");
    println!("==================================");

    let sender_keys = Keys::generate();
    let receiver_keys = Keys::generate();

    println!("Sender public key: {}", sender_keys.public_key());
    println!("Receiver public key: {}", receiver_keys.public_key());
    println!();

    let relays = vec![
        "wss://relay.damus.io".to_string(),
        "wss://nostr.chaima.info".to_string(),
    ];

    let config = Config::new(
        sender_keys.secret_key().display_secret().to_string(),
        relays,
    )
    .with_nip44_encryption(receiver_keys.public_key().to_string());

    let client = NostrSentryClient::new(config).await?;

    println!("Publishing encrypted events...");

    let event = Event::new()
        .with_message("This is an encrypted error message")
        .with_level(Level::Error)
        .with_tag("component", "encrypted-example")
        .with_tag("encrypted", "true")
        .with_extra("secret_data", serde_json::json!("sensitive information"));

    let event_id = client.capture_event(event).await?;
    println!("Published encrypted event: {}", event_id);

    client
        .capture_message("Encrypted message from SentryStr")
        .await?;
    client
        .capture_error("Encrypted error from SentryStr")
        .await?;

    println!();
    println!("Demonstrating direct encryption/decryption...");

    let test_message = "Secret log message: authentication failed for user admin";
    let encrypted = EncryptionHelper::encrypt_nip44(
        sender_keys.secret_key(),
        &receiver_keys.public_key(),
        test_message,
    )?;

    println!("Original: {}", test_message);
    println!("Encrypted: {}", encrypted);

    let decrypted = EncryptionHelper::decrypt_nip44(
        receiver_keys.secret_key(),
        &sender_keys.public_key(),
        &encrypted,
    )?;

    println!("Decrypted: {}", decrypted);

    client.disconnect().await?;
    Ok(())
}
