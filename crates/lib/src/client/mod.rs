//! The user client.

use crate::prelude::*;
use arrayref::array_ref;
use secp256k1::{rand, KeyPair, Secp256k1};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::{broadcast, mpsc};
use tokio::task::JoinHandle;

/// The user client.
pub struct Client {
    /// Secret key and public key for this client.
    keys: KeyPair,
    /// Subscriptions for this client.
    subscriptions: HashMap<subscription::ID, subscription::Subscription>,
    /// Relays for this client.
    relays: HashMap<relay::URL, Arc<Mutex<relay::Relay>>>,
    /// Message sender.
    /// The `Sender` is used for sending messages *to* relays trough their `Receiver`.
    _sender: (broadcast::Sender<message::Message>, broadcast::Receiver<message::Message>),
    /// Message receiver.
    /// The `Sender` is used for sending messages *from* relays to this `Receiver`.
    _receiver: (mpsc::Sender<message::Message>, mpsc::Receiver<message::Message>),
}

impl Client {
    /// Create a user client from a keypair.
    fn from_keys(keys: KeyPair) -> Self {
        Self {
            keys,
            subscriptions: HashMap::new(),
            relays: HashMap::new(),
            _sender: broadcast::channel(5000),
            _receiver: mpsc::channel(5000),
        }
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

    /// Add this to relays to send messages.
    pub fn get_outgoing_receiver(&self) -> broadcast::Receiver<message::Message> {
        self._sender.0.subscribe()
    }

    pub fn get_outgoing_sender(&mut self) -> &mut broadcast::Sender<message::Message> {
        &mut self._sender.0
    }

    /// Add this to relays to receive messages.
    pub fn get_incoming_sender(&self) -> mpsc::Sender<message::Message> {
        self._receiver.0.clone()
    }

    /// Use this to read messages from relays.
    pub fn get_incoming_receiver(&mut self) -> &mut mpsc::Receiver<message::Message> {
        &mut self._receiver.1
    }

    /// Add a relay.
    pub fn add_relay(&mut self, url: relay::URL) {
        let relay = relay::Relay::new(url.clone(), self.get_outgoing_receiver(), self.get_incoming_sender());
        let relay = Arc::new(Mutex::new(relay));

        self.relays.insert(url.clone(), relay);
    }

    /// Connect to all relays.
    pub fn connect_relays(&mut self) -> Result<Vec<JoinHandle<()>>, Error> {
        let mut pool = Vec::new();

        for relay in self.relays.values().cloned() {
            pool.push(tokio::spawn(async move {
                relay.lock().await.open().await.expect("relay open");
            }));
        }

        Ok(pool)
    }

    pub async fn receive_message(&mut self) -> Option<message::Message> {
        self.get_incoming_receiver().recv().await
    }

    pub async fn send_message(&mut self, message: message::Message) -> Result<(), Error> {
        self.get_outgoing_sender().send(message)?;

        Ok(())
    }

    /// Create a signed [`Event`].
    pub fn create_event(&self, created_at: i64, kind: event::Kind, tags: Vec<event::Tag>, content: String) -> Result<event::Event, Error> {
        let tmp = self.keys.public_key().serialize();
        let pubkey = *array_ref![tmp, 1, 32];

        let event = event::Event::try_new(&self.keys, pubkey, created_at, kind, tags, content)?;

        Ok(event)
    }
}
