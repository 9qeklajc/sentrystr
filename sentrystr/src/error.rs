use thiserror::Error;

#[derive(Error, Debug)]
pub enum SentryStrError {
    #[error("Nostr error: {0}")]
    Nostr(#[from] nostr::key::Error),

    #[error("Nostr SDK error: {0}")]
    NostrSdk(#[from] nostr_sdk::client::Error),

    #[error("Nostr event builder error: {0}")]
    NostrEventBuilder(#[from] nostr::event::builder::Error),

    #[error("NIP-44 encryption error: {0}")]
    Nip44Encryption(#[from] nostr::nips::nip44::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Publishing error: {0}")]
    Publishing(String),
}
