use crate::{Result, SentryStrError};
use nostr::prelude::*;

pub struct EncryptionHelper;

impl EncryptionHelper {
    pub fn encrypt_nip44(
        sender_secret_key: &SecretKey,
        recipient_public_key: &PublicKey,
        content: &str,
    ) -> Result<String> {
        let encrypted = nostr::nips::nip44::encrypt(
            sender_secret_key,
            recipient_public_key,
            content,
            nostr::nips::nip44::Version::V2,
        )?;
        Ok(encrypted)
    }

    pub fn decrypt_nip44(
        receiver_secret_key: &SecretKey,
        sender_public_key: &PublicKey,
        encrypted_content: &str,
    ) -> Result<String> {
        let decrypted =
            nostr::nips::nip44::decrypt(receiver_secret_key, sender_public_key, encrypted_content)?;
        Ok(decrypted)
    }

    pub fn generate_shared_point(
        _secret_key: &SecretKey,
        public_key: &PublicKey,
    ) -> Result<PublicKey> {
        let shared_point = *public_key;
        Ok(shared_point)
    }
}

pub fn validate_encryption_keys(sender_keys: &Keys, recipient_pubkey: &PublicKey) -> Result<()> {
    if sender_keys.public_key() == *recipient_pubkey {
        return Err(SentryStrError::Config(
            "Cannot encrypt to yourself - sender and recipient keys are the same".to_string(),
        ));
    }
    Ok(())
}
