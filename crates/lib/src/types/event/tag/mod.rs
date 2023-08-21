//! The tag object.

use crate::relay;

/// A tag, attached to an event object.
#[derive(Debug, Clone)]
pub enum Tag {
    /// 32-bytes hex of the id of another event.
    E { event_id: [u8; 32], relay_url: Option<relay::URL> },
    /// 32-bytes hex of a pubkey.
    P { pubkey: [u8; 32], relay_url: Option<relay::URL> },
    /// Unidentified tag.
    Other { data: String },
}
