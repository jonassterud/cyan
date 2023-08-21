//! The user client.

use crate::prelude::{Error, Event, Kind, Tag};
use arrayref::array_ref;
use secp256k1::{rand, KeyPair, Secp256k1};

/// The user client.
pub struct Client {
    /// Secret key and public key for this client.
    keys: KeyPair,
}

impl Client {
    /// Create a user client from a keypair.
    fn from_keys(keys: KeyPair) -> Self {
        Self { keys }
    }

    /// Create a new user client with a random key.
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let (secret_key, _) = secp.generate_keypair(&mut rand::thread_rng());
        let key_pair = KeyPair::from_secret_key(&secp, &secret_key);

        Self::from_keys(key_pair)
    }

    /// Create a user client with an existing secret key.
    pub fn from_secret_key(secret_key: &[u8]) -> Result<Self, Error> {
        let secp = Secp256k1::new();
        let key_pair = KeyPair::from_seckey_slice(&secp, secret_key)?;

        Ok(Self::from_keys(key_pair))
    }

    /// Create a signed [`Event`].
    pub fn create_event(&self, created_at: i64, kind: Kind, tags: Vec<Tag>, content: String) -> Result<Event, Error> {
        let tmp = self.keys.public_key().serialize();
        let pubkey = *array_ref![tmp, 1, 32];

        let event = Event::try_new(&self.keys, pubkey, created_at, kind, tags, content)?;

        Ok(event)
    }
}
