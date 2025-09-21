use nostr::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub secret_key: String,
    pub relays: Vec<String>,
    pub encrypt_events: bool,
    pub recipient_pubkey: Option<String>,
    pub event_kind: u16,
    pub tags: Option<Vec<Tag>>,
    pub encryption_version: EncryptionVersion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionVersion {
    None,
    Nip44V2,
}

impl Config {
    pub fn new(secret_key: String, relays: Vec<String>) -> Self {
        Self {
            secret_key,
            relays,
            encrypt_events: false,
            recipient_pubkey: None,
            event_kind: 9898,
            tags: None,
            encryption_version: EncryptionVersion::None,
        }
    }

    pub fn with_encryption(mut self, recipient_pubkey: String) -> Self {
        self.encrypt_events = true;
        self.recipient_pubkey = Some(recipient_pubkey);
        self.encryption_version = EncryptionVersion::Nip44V2;
        self
    }

    pub fn with_nip44_encryption(mut self, recipient_pubkey: String) -> Self {
        self.encrypt_events = true;
        self.recipient_pubkey = Some(recipient_pubkey);
        self.encryption_version = EncryptionVersion::Nip44V2;
        self
    }

    pub fn with_tags(mut self, tags: Vec<Tag>) -> Self {
        self.tags = Some(tags);
        self
    }

    pub fn get_keys(&self) -> Result<Keys, nostr::key::Error> {
        Keys::parse(&self.secret_key)
    }

    pub fn get_recipient_pubkey(&self) -> Result<Option<PublicKey>, nostr::key::Error> {
        match &self.recipient_pubkey {
            Some(pubkey) => Ok(Some(PublicKey::parse(pubkey)?)),
            None => Ok(None),
        }
    }
}
