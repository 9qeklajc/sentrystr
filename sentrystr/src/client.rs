use crate::{
    validate_encryption_keys, Config, DirectMessageSender, EncryptionVersion, Event, MessageEvent,
    Result, SentryStrError,
};
use chrono::Utc;
use nostr::prelude::*;
use nostr_sdk::prelude::*;

/// Main client for sending events to Nostr relays with optional direct messaging.
///
/// # Examples
///
/// ```rust
/// use sentrystr::{Config, NostrSentryClient, Event, Level};
/// use nostr::Keys;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let keys = Keys::generate();
/// let config = Config::new(keys.secret_key().display_secret().to_string(), vec!["wss://relay.damus.io".to_string()]);
/// let client = NostrSentryClient::new(config).await?;
///
/// let event = Event::new().with_message("Error occurred").with_level(Level::Error);
/// client.capture_event(event).await?;
/// # Ok(())
/// # }
/// ```
pub struct NostrSentryClient {
    client: Client,
    config: Config,
    keys: Keys,
    dm_sender: Option<DirectMessageSender>,
}

impl NostrSentryClient {
    /// Creates a new NostrSentryClient with the given configuration.
    ///
    /// This will connect to all specified relays automatically.
    pub async fn new(config: Config) -> Result<Self> {
        let keys = config.get_keys()?;
        let client = Client::new(keys.clone());

        for relay in &config.relays {
            client.add_relay(relay).await?;
        }

        client.connect().await;

        Ok(Self {
            client,
            config,
            keys,
            dm_sender: None,
        })
    }

    pub async fn capture_event(&self, event: Event) -> Result<EventId> {
        let content = serde_json::to_string(&event)?;

        let nostr_event = if self.config.encrypt_events {
            match self.config.encryption_version {
                EncryptionVersion::None => {
                    return Err(SentryStrError::Config(
                        "Encryption enabled but version not specified".to_string(),
                    ));
                }
                EncryptionVersion::Nip44V2 => {
                    if let Some(recipient_pubkey) = self.config.get_recipient_pubkey()? {
                        validate_encryption_keys(&self.keys, &recipient_pubkey)?;

                        let encrypted_content = nostr::nips::nip44::encrypt(
                            self.keys.secret_key(),
                            &recipient_pubkey,
                            &content,
                            nostr::nips::nip44::Version::V2,
                        )?;

                        let mut builder = EventBuilder::new(
                            Kind::Custom(self.config.event_kind),
                            encrypted_content,
                        );

                        let mut all_tags = event.nostr_tags.clone();
                        if let Some(ref config_tags) = self.config.tags {
                            all_tags.extend(config_tags.clone());
                        }
                        if !all_tags.is_empty() {
                            builder = builder.tags(all_tags);
                        }

                        builder.sign_with_keys(&self.keys)?
                    } else {
                        return Err(SentryStrError::Config(
                            "Encryption enabled but no recipient public key provided".to_string(),
                        ));
                    }
                }
            }
        } else {
            let mut builder = EventBuilder::new(Kind::Custom(self.config.event_kind), content);

            let mut all_tags = event.nostr_tags.clone();
            if let Some(ref config_tags) = self.config.tags {
                all_tags.extend(config_tags.clone());
            }
            if !all_tags.is_empty() {
                builder = builder.tags(all_tags);
            }

            builder.sign_with_keys(&self.keys)?
        };

        let output = self.client.send_event(&nostr_event).await?;

        // Send direct message if configured
        if let Some(ref dm_sender) = self.dm_sender {
            let message_event = MessageEvent {
                event: event.clone(),
                author: self.keys.public_key(),
                nostr_event_id: output.val,
                received_at: Utc::now(),
            };

            if let Err(e) = dm_sender.send_message_for_event(&message_event).await {
                eprintln!("Failed to send direct message: {}", e);
            }
        }

        Ok(output.val)
    }

    pub async fn capture_message(&self, message: impl Into<String>) -> Result<EventId> {
        let event = Event::new().with_message(message);
        self.capture_event(event).await
    }

    pub async fn capture_error(&self, error: impl Into<String>) -> Result<EventId> {
        let event = Event::new()
            .with_message(error)
            .with_level(crate::event::Level::Error);
        self.capture_event(event).await
    }

    pub async fn disconnect(&self) -> Result<()> {
        self.client.disconnect().await;
        Ok(())
    }

    pub fn add_tag(&mut self, tag: Tag) {
        match self.config.tags {
            Some(ref mut tags) => tags.push(tag),
            None => self.config.tags = Some(vec![tag]),
        }
    }

    pub fn with_direct_messaging(mut self, dm_sender: DirectMessageSender) -> Self {
        self.dm_sender = Some(dm_sender);
        self
    }

    pub fn set_direct_messaging(&mut self, dm_sender: DirectMessageSender) {
        self.dm_sender = Some(dm_sender);
    }

    pub fn remove_direct_messaging(&mut self) {
        self.dm_sender = None;
    }

    pub async fn send_direct_message(&self, content: &str) -> Result<()> {
        if let Some(ref dm_sender) = self.dm_sender {
            dm_sender.send_custom_message(content).await
        } else {
            Err(SentryStrError::Config(
                "Direct messaging not configured".to_string(),
            ))
        }
    }
}
