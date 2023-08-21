mod serde_custom;

use crate::types::Event;

/// The differente messages a client can send or receive.
pub enum Message {
    EVENT { event: Event },
    REQ { subscription_id: String, filters: String },
    CLOSE { subscription_id: String },
    OK { event_id: i32, status: bool, message: String },
    EOSE { subscription_id: i32 },
    NOTICE { message: String },
}
