//! The relay.

#![allow(non_camel_case_types)]

use tokio::net::TcpStream;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::prelude::*;

pub type URL = String;

/// Used to connect to and communicate with relays.
pub struct Relay {
    /// The URL - without the protocol part.
    ///
    /// `example.com` not `https://example.com`.
    pub url: URL,
    /// Outgoing messages.
    pub outgoing: broadcast::Receiver<message::Message>,
    /// Incoming messages.
    pub incoming: mpsc::Sender<message::Message>,
    /// WebSocket stream
    pub stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl Relay {
    /// Create a new [`Relay`].
    pub fn new(url: URL, outgoing: broadcast::Receiver<message::Message>, incoming: mpsc::Sender<message::Message>) -> Self {
        Self {
            url,
            outgoing,
            incoming,
            stream: None,
        }
    }

    /// Open a websocket connection to the relay.
    pub async fn open(&mut self) -> Result<(), Error> {
        let ws_url = format!("wss://{}", self.url);
        let ws_stream = tokio_tungstenite::connect_async(ws_url).await?;
        self.stream = Some(ws_stream.0);

        Ok(())
    }

    /// Close a websocket connection to the relay.
    pub fn close(&self) -> Result<(), Error> {
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
