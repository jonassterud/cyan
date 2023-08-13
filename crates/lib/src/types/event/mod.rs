use super::Tag;
use crate::{error::Error, helpers};
use secp256k1::{
    hashes::{sha256, Hash},
    KeyPair, Secp256k1,
};
use serde_json::json;

/// The Nostr event object.
pub struct Event {
    /// 32-bytes lowercase hex-encoded sha256 of the serialized event data.
    pub id: [u8; 32],
    /// 32-bytes lowercase hex-encoded public key of the event creator.
    pub pubkey: [u8; 32],
    /// Unix timestamp in seconds,
    pub created_at: i64,
    /// Event kind.
    pub kind: i32,
    /// List of tags.
    pub tags: Vec<Tag>,
    /// Arbitrary string.
    pub content: String,
    /// 64-bytes hex of the signature of the sha256 hash of the serialized event data.
    pub sig: [u8; 64],
}

/// The Nostr event object, except its missing `id` and `sig`.
struct UnsignedEvent {
    /// 32-bytes lowercase hex-encoded public key of the event creator.
    pub pubkey: [u8; 32],
    /// Unix timestamp in seconds,
    pub created_at: i64,
    /// Event kind.
    pub kind: i32,
    /// List of tags.
    pub tags: Vec<Tag>,
    /// Arbitrary string.
    pub content: String,
}

impl Event {
    /// Create a signed event.
    pub fn try_new(keys: &KeyPair, pubkey: [u8; 32], kind: i32, tags: Vec<Tag>, content: String) -> Result<Self, Error> {
        let created_at = helpers::get_unix_timestamp()?;
        let unsigned_event = UnsignedEvent::new(pubkey, created_at, kind, tags, content);
        let id = unsigned_event.get_id();
        let sig = unsigned_event.get_sig(&id, keys)?;

        Ok(Self {
            id,
            pubkey: unsigned_event.pubkey,
            created_at: unsigned_event.created_at,
            kind: unsigned_event.kind,
            tags: unsigned_event.tags,
            content: unsigned_event.content,
            sig,
        })
    }
}

impl UnsignedEvent {
    /// Create a unsigned event.
    fn new(pubkey: [u8; 32], created_at: i64, kind: i32, tags: Vec<Tag>, content: String) -> Self {
        Self { pubkey, created_at, kind, tags, content }
    }

    /// Get the serialized event data.
    fn get_id(&self) -> [u8; 32] {
        let serialized_event = json!([0, format!("{:x?}", self.pubkey), self.created_at, self.kind, self.tags, self.content]).to_string();
        let serialized_event_hash = sha256::Hash::hash(serialized_event.as_bytes());

        *serialized_event_hash.as_ref() // I think this is OK?
    }

    /// Sign the serialized event data with a private key.
    fn get_sig(&self, id: &[u8], keypair: &KeyPair) -> Result<[u8; 64], Error> {
        let message = secp256k1::Message::from_slice(id)?;
        let sig = Secp256k1::new().sign_schnorr(&message, keypair);

        Ok(*sig.as_ref()) // I think this is OK?
    }
}
