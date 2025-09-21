use crate::{EncryptionHelper, Event, Result, SentryStrError};
use chrono::{DateTime, Utc};
use nostr::prelude::*;
use nostr_sdk::prelude::*;
use serde_json;

#[derive(Debug, Clone)]
pub struct DirectMessageConfig {
    pub recipient_pubkey: PublicKey,
    pub min_level: Option<crate::Level>,
    pub use_nip17: bool,
}

#[derive(Debug)]
pub struct MessageEvent {
    pub event: Event,
    pub author: PublicKey,
    pub nostr_event_id: EventId,
    pub received_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct DirectMessageSender {
    client: Client,
    keys: Keys,
    config: DirectMessageConfig,
}

impl DirectMessageSender {
    pub fn new(client: Client, keys: Keys, config: DirectMessageConfig) -> Self {
        Self {
            client,
            keys,
            config,
        }
    }

    pub async fn send_message_for_event(&self, event: &MessageEvent) -> Result<()> {
        if !self.should_send_for_level(&event.event.level) {
            return Ok(());
        }

        let event_json = serde_json::to_string_pretty(&event.event)?;
        let message_content = format!(
            "SentryStr Alert\n\nEvent ID: {}\nAuthor: {}\nTimestamp: {}\nLevel: {:?}\n\nEvent Data:\n{}",
            event.nostr_event_id,
            event.author,
            event.event.timestamp,
            event.event.level,
            event_json
        );

        if self.config.use_nip17 {
            self.send_nip17_message(&message_content).await
        } else {
            self.send_nip44_message(&message_content).await
        }
    }

    pub async fn send_custom_message(&self, content: &str) -> Result<()> {
        if self.config.use_nip17 {
            self.send_nip17_message(content).await
        } else {
            self.send_nip44_message(content).await
        }
    }

    fn should_send_for_level(&self, event_level: &crate::Level) -> bool {
        if let Some(ref min_level) = self.config.min_level {
            match (min_level, event_level) {
                (crate::Level::Debug, _) => true,
                (crate::Level::Info, crate::Level::Debug) => false,
                (crate::Level::Info, _) => true,
                (crate::Level::Warning, crate::Level::Debug | crate::Level::Info) => false,
                (crate::Level::Warning, _) => true,
                (
                    crate::Level::Error,
                    crate::Level::Debug | crate::Level::Info | crate::Level::Warning,
                ) => false,
                (crate::Level::Error, _) => true,
                (crate::Level::Fatal, crate::Level::Fatal) => true,
                (crate::Level::Fatal, _) => false,
            }
        } else {
            true
        }
    }

    async fn send_nip17_message(&self, content: &str) -> Result<()> {
        const MAX_RETRIES: u32 = 3;
        const BASE_DELAY_MS: u64 = 1000;

        for attempt in 0..MAX_RETRIES {
            match self
                .client
                .send_private_msg(self.config.recipient_pubkey, content, [])
                .await
            {
                Ok(_) => {
                    if attempt > 0 {
                        eprintln!("Successfully sent NIP-17 message after {} retries", attempt);
                    }
                    return Ok(());
                }
                Err(e) => {
                    eprintln!("NIP-17 send attempt {} failed: {}", attempt + 1, e);
                    if attempt < MAX_RETRIES - 1 {
                        let delay =
                            std::time::Duration::from_millis(BASE_DELAY_MS * (1 << attempt));
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }

        Err(SentryStrError::Config(
            "Failed to send NIP-17 message after retries".to_string(),
        ))
    }

    async fn send_nip44_message(&self, content: &str) -> Result<()> {
        const MAX_RETRIES: u32 = 3;
        const BASE_DELAY_MS: u64 = 1000;

        for attempt in 0..MAX_RETRIES {
            let encrypted_content = EncryptionHelper::encrypt_nip44(
                self.keys.secret_key(),
                &self.config.recipient_pubkey,
                content,
            )?;

            let dm_event = EventBuilder::new(Kind::EncryptedDirectMessage, encrypted_content)
                .tag(Tag::public_key(self.config.recipient_pubkey))
                .sign_with_keys(&self.keys)?;

            match self.client.send_event(&dm_event).await {
                Ok(_) => {
                    if attempt > 0 {
                        eprintln!("Successfully sent NIP-44 message after {} retries", attempt);
                    }
                    return Ok(());
                }
                Err(e) => {
                    eprintln!("NIP-44 send attempt {} failed: {}", attempt + 1, e);
                    if attempt < MAX_RETRIES - 1 {
                        let delay =
                            std::time::Duration::from_millis(BASE_DELAY_MS * (1 << attempt));
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }

        Err(SentryStrError::Config(
            "Failed to send NIP-44 message after retries".to_string(),
        ))
    }
}

pub struct DirectMessageBuilder {
    client: Option<Client>,
    keys: Option<Keys>,
    recipient_pubkey: Option<PublicKey>,
    min_level: Option<crate::Level>,
    use_nip17: bool,
}

impl DirectMessageBuilder {
    pub fn new() -> Self {
        Self {
            client: None,
            keys: None,
            recipient_pubkey: None,
            min_level: None,
            use_nip17: false,
        }
    }

    pub fn with_client(mut self, client: Client) -> Self {
        self.client = Some(client);
        self
    }

    pub fn with_keys(mut self, keys: Keys) -> Self {
        self.keys = Some(keys);
        self
    }

    pub fn with_recipient(mut self, pubkey: PublicKey) -> Self {
        self.recipient_pubkey = Some(pubkey);
        self
    }

    pub fn with_min_level(mut self, level: crate::Level) -> Self {
        self.min_level = Some(level);
        self
    }

    pub fn with_nip17(mut self, use_nip17: bool) -> Self {
        self.use_nip17 = use_nip17;
        self
    }

    pub fn build(self) -> Result<DirectMessageSender> {
        let client = self.client.ok_or_else(|| {
            SentryStrError::Config("Client is required for DirectMessageSender".to_string())
        })?;

        let keys = self.keys.ok_or_else(|| {
            SentryStrError::Config("Keys are required for DirectMessageSender".to_string())
        })?;

        let recipient_pubkey = self.recipient_pubkey.ok_or_else(|| {
            SentryStrError::Config(
                "Recipient pubkey is required for DirectMessageSender".to_string(),
            )
        })?;

        let config = DirectMessageConfig {
            recipient_pubkey,
            min_level: self.min_level,
            use_nip17: self.use_nip17,
        };

        Ok(DirectMessageSender::new(client, keys, config))
    }
}

impl Default for DirectMessageBuilder {
    fn default() -> Self {
        Self::new()
    }
}
