mod serde_custom;

/// A possible tag, attached to an event object.
#[derive(Debug, Clone)]
pub enum Tag {
    /// 32-bytes hex of the id of another event.
    E { event_id: [u8; 32], relay_url: Option<String> },
    /// 32-bytes hex of a pubkey.
    P { pubkey: [u8; 32], relay_url: Option<String> },
    /// Unidentified tag.
    Other { data: String },
}
