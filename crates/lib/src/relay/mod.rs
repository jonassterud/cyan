//! The relay.

#![allow(non_camel_case_types)]

use crate::prelude::*;

pub type URL = String;

/// Used to connect to and communicate with relays.
pub struct Relay {
    /// The URL - without the protocol part.
    ///
    /// `example.com` not `https://example.com`.
    pub url: URL,
}

impl Relay {
    /// Create a new [`Relay`].
    pub fn new(url: URL) -> Self {
        Self { url }
    }

    /// Open a websocket connection to the relay.
    pub fn open() -> Result<(), Error> {
        todo!()
    }

    /// Close a websocket connection to the relay.
    pub fn close() -> Result<(), Error> {
        todo!()
    }

    /// Send a message to the relay.
    pub fn send(message: message::Message) -> Result<(), Error> {
        todo!()
    }

    /// Read a message from the queue.
    pub fn receive() -> Result<message::Message, Error> {
        todo!();
    }
}
