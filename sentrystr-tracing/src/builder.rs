use crate::{Result, SentryStrLayer, TracingError};
use nostr::prelude::*;
use nostr_sdk::prelude::*;
use sentrystr::{Config, DirectMessageBuilder, NostrSentryClient};
use tracing_subscriber::prelude::*;

/// Builder for configuring SentryStr tracing integration.
///
/// # Examples
///
/// ```rust
/// use sentrystr_tracing::SentryStrTracingBuilder;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Basic setup
/// SentryStrTracingBuilder::new()
///     .with_generated_keys_and_relays(vec!["wss://relay.damus.io".to_string()])
///     .with_min_level(tracing::Level::INFO)
///     .init()
///     .await?;
/// # Ok(())
/// # }
/// ```
pub struct SentryStrTracingBuilder {
    config: Option<Config>,
    dm_config: Option<DirectMessageConfig>,
    min_level: Option<tracing::Level>,
    include_fields: bool,
    include_metadata: bool,
}

/// Configuration for direct message alerts in tracing.
///
/// # Examples
///
/// ```rust
/// use sentrystr_tracing::builder::DirectMessageConfig;
/// use sentrystr::Level;
/// use nostr::Keys;
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let recipient = Keys::generate().public_key();
/// let relays = vec!["wss://relay.damus.io".to_string()];
///
/// let dm_config = DirectMessageConfig::new(recipient, relays)
///     .with_min_level(Level::Error)
///     .with_nip17(true);
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct DirectMessageConfig {
    pub recipient_pubkey: PublicKey,
    pub min_level: Option<sentrystr::Level>,
    pub use_nip17: bool,
    pub relays: Vec<String>,
}

impl SentryStrTracingBuilder {
    /// Creates a new SentryStrTracingBuilder with default settings.
    pub fn new() -> Self {
        Self {
            config: None,
            dm_config: None,
            min_level: None,
            include_fields: true,
            include_metadata: true,
        }
    }

    pub fn with_config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    pub fn with_secret_key_and_relays(mut self, secret_key: String, relays: Vec<String>) -> Self {
        self.config = Some(Config::new(secret_key, relays));
        self
    }

    pub fn with_generated_keys_and_relays(mut self, relays: Vec<String>) -> Self {
        let keys = Keys::generate();
        self.config = Some(Config::new(
            keys.secret_key().display_secret().to_string(),
            relays,
        ));
        self
    }

    pub fn with_direct_messaging(mut self, dm_config: DirectMessageConfig) -> Self {
        self.dm_config = Some(dm_config);
        self
    }

    pub fn with_dm_recipient(mut self, recipient_pubkey: PublicKey, relays: Vec<String>) -> Self {
        self.dm_config = Some(DirectMessageConfig {
            recipient_pubkey,
            min_level: None,
            use_nip17: true,
            relays,
        });
        self
    }

    pub fn with_min_level(mut self, level: tracing::Level) -> Self {
        self.min_level = Some(level);
        self
    }

    pub fn with_fields(mut self, include: bool) -> Self {
        self.include_fields = include;
        self
    }

    pub fn with_metadata(mut self, include: bool) -> Self {
        self.include_metadata = include;
        self
    }

    pub async fn build(self) -> Result<SentryStrLayer> {
        let config = self
            .config
            .ok_or_else(|| TracingError::Config("SentryStr config is required".to_string()))?;

        let client = NostrSentryClient::new(config).await?;

        let mut layer = SentryStrLayer::new(client)
            .with_fields(self.include_fields)
            .with_metadata(self.include_metadata);

        if let Some(min_level) = self.min_level {
            layer = layer.with_min_level(min_level);
        }

        if let Some(dm_config) = self.dm_config {
            let dm_keys = Keys::generate();
            let dm_client = Client::new(dm_keys.clone());

            for relay in &dm_config.relays {
                dm_client.add_relay(relay).await?;
            }
            dm_client.connect().await;

            let dm_sender = DirectMessageBuilder::new()
                .with_client(dm_client)
                .with_keys(dm_keys)
                .with_recipient(dm_config.recipient_pubkey)
                .with_min_level(
                    dm_config
                        .min_level
                        .unwrap_or(sentrystr::Level::Warning),
                )
                .with_nip17(dm_config.use_nip17)
                .build()?;

            layer = layer.with_direct_messaging(dm_sender);
        }

        Ok(layer)
    }

    pub async fn init(self) -> Result<()> {
        let layer = self.build().await?;

        tracing_subscriber::registry()
            .with(layer)
            .with(tracing_subscriber::fmt::layer())
            .init();

        Ok(())
    }

    pub async fn init_with_env_filter(self, env_filter: &str) -> Result<()> {
        let layer = self.build().await?;

        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(env_filter))
            .with(layer)
            .with(tracing_subscriber::fmt::layer())
            .init();

        Ok(())
    }
}

impl Default for SentryStrTracingBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl DirectMessageConfig {
    pub fn new(recipient_pubkey: PublicKey, relays: Vec<String>) -> Self {
        Self {
            recipient_pubkey,
            min_level: None,
            use_nip17: true,
            relays,
        }
    }

    pub fn with_min_level(mut self, level: sentrystr::Level) -> Self {
        self.min_level = Some(level);
        self
    }

    pub fn with_nip17(mut self, use_nip17: bool) -> Self {
        self.use_nip17 = use_nip17;
        self
    }
}
