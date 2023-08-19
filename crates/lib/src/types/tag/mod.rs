use serde::{ser, Serialize, Serializer};

/// A possible tag, attached to an event object.
#[derive(Clone)]
pub enum Tag {
    /// 32-bytes hex of the id of another event.
    E { event_id: [u8; 32], relay_url: Option<String> },
    /// 32-bytes hex of a pubkey.
    P { pubkey: [u8; 32], relay_url: Option<String> },
    /// Unidentified tag.
    Other { data: String },
}

impl Serialize for Tag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::E { event_id, relay_url } => ("e", event_id, relay_url).serialize(serializer),
            Self::P { pubkey, relay_url } => ("p", pubkey, relay_url).serialize(serializer),
            Self::Other { data } => Err(ser::Error::custom(format!("unidentified tag: {:?}", data))),
        }
    }
}
