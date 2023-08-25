use crate::prelude::*;

/// Error type.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid system time")]
    SystemTime(#[from] std::time::SystemTimeError),
    #[error("failed casting integer")]
    TryFromInt(#[from] std::num::TryFromIntError),
    #[error("failed hashing / signing")]
    Secp256k1(#[from] secp256k1::Error),
    #[error("failed serializing or deserializing")]
    SerdeJson(#[from] serde_json::Error),
    #[error("wrong value (expected {expected:?}, found {found:?})")]
    ExpectedFound { expected: String, found: String },
    #[error("decoding hex failed")]
    FromHex(#[from] hex::FromHexError),
    #[error("websocket failed")]
    Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("broadcast failed")]
    Broadcast(#[from] tokio::sync::broadcast::error::SendError<message::Message>),
    #[error("something went wrong: {0}")]
    Custom(String),
}
