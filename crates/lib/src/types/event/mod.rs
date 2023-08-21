mod serde_custom;

use crate::{
    error::Error,
    types::{Kind, Tag},
};
use secp256k1::{
    hashes::{sha256, Hash},
    schnorr::Signature,
    KeyPair, Secp256k1, XOnlyPublicKey,
};
use serde_json::json;

/// The Nostr event object.
#[derive(Debug)]
pub struct Event {
    /// 32-bytes lowercase hex-encoded sha256 of the serialized event data.
    pub id: [u8; 32],
    /// 32-bytes lowercase hex-encoded public key of the event creator.
    pub pubkey: [u8; 32],
    /// Unix timestamp in seconds,
    pub created_at: i64,
    /// Event kind.
    pub kind: Kind,
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
    pub kind: Kind,
    /// List of tags.
    pub tags: Vec<Tag>,
    /// Arbitrary string.
    pub content: String,
}

impl Event {
    /// Create a signed event.
    pub fn try_new(keys: &KeyPair, pubkey: [u8; 32], created_at: i64, kind: Kind, tags: Vec<Tag>, content: String) -> Result<Self, Error> {
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

    /// Check the id of an event.
    pub fn check_id(&self) -> Result<(), Error> {
        let unsigned_event = UnsignedEvent::from_event(self);
        let expected_id = unsigned_event.get_id();

        match self.id == unsigned_event.get_id() {
            true => Ok(()),
            false => Err(Error::ExpectedFound {
                expected: hex::encode(expected_id),
                found: hex::encode(self.id),
            }),
        }
    }

    /// Check the signature of an event.
    pub fn check_sig(&self) -> Result<(), Error> {
        let signature = Signature::from_slice(&self.sig)?;
        let message = secp256k1::Message::from_slice(&self.id).unwrap();
        let pubkey = XOnlyPublicKey::from_slice(&self.pubkey)?;

        Secp256k1::new().verify_schnorr(&signature, &message, &pubkey)?;

        Ok(())
    }

    /// Serialize as JSON.
    pub fn serialize(&self) -> Result<String, Error> {
        let json = serde_json::to_string(self)?;

        Ok(json)
    }

    /// Deserialize from JSON.
    pub fn deserialize(data: &[u8]) -> Result<Self, Error> {
        let event = serde_json::from_slice(data)?;

        Ok(event)
    }
}

impl UnsignedEvent {
    /// Create a unsigned event.
    fn new(pubkey: [u8; 32], created_at: i64, kind: Kind, tags: Vec<Tag>, content: String) -> Self {
        Self { pubkey, created_at, kind, tags, content }
    }

    /// Create an unsigned event from a signed event.
    fn from_event(event: &Event) -> Self {
        Self {
            pubkey: event.pubkey,
            created_at: event.created_at,
            kind: event.kind.clone(),
            tags: event.tags.clone(),
            content: event.content.clone(),
        }
    }

    /// Get the serialized event data.
    fn get_id(&self) -> [u8; 32] {
        let serialized_event = json!([0, hex::encode(&self.pubkey), self.created_at, self.kind, self.tags, self.content]).to_string();
        let serialized_event_hash = sha256::Hash::hash(serialized_event.as_bytes());

        *serialized_event_hash.as_ref() // I think this is OK?
    }

    /// Sign the serialized event data with a private key.
    fn get_sig(&self, id: &[u8; 32], keys: &KeyPair) -> Result<[u8; 64], Error> {
        let message = secp256k1::Message::from_slice(id).unwrap();
        let sig = Secp256k1::new().sign_schnorr(&message, keys);

        Ok(*sig.as_ref()) // I think this is OK?
    }
}
