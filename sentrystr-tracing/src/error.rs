use thiserror::Error;

#[derive(Error, Debug)]
pub enum TracingError {
    #[error("SentryStr core error: {0}")]
    SentryStr(#[from] sentrystr::SentryStrError),

    #[error("Nostr SDK error: {0}")]
    NostrSdk(#[from] nostr_sdk::client::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Initialization error: {0}")]
    Init(String),

    #[error("Field extraction error: {0}")]
    FieldExtraction(String),
}
