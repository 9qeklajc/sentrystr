use thiserror::Error;

#[derive(Error, Debug)]
pub enum CollectorError {
    #[error("Nostr error: {0}")]
    Nostr(#[from] nostr::key::Error),

    #[error("Nostr SDK error: {0}")]
    NostrSdk(#[from] nostr_sdk::client::Error),

    #[error("Nostr event builder error: {0}")]
    EventBuilder(#[from] nostr::event::builder::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("SentryStr core error: {0}")]
    SentryStr(#[from] sentrystr::SentryStrError),

    #[error("Collection error: {0}")]
    Collection(String),

    #[error("Filter error: {0}")]
    Filter(String),
}
