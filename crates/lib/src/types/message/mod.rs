mod serde_custom;

use serde_json::Value;

use crate::error::Error;
use crate::types::Event;

/// The differente messages a client can send or receive.
#[derive(Debug)]
pub enum Message {
    EVENT { subscription_id: String, event: Event },
    REQ { subscription_id: String, filters: String },
    CLOSE { subscription_id: String },
    OK { event_id: i32, status: bool, message: String },
    EOSE { subscription_id: i32 },
    NOTICE { message: String },
}

impl Message {
    /// Serialize as JSON.
    pub fn serialize(&self) -> Result<Value, Error> {
        let json = serde_json::to_value(self)?;

        Ok(json)
    }

    /// Deserialize from JSON.
    pub fn deserialize<T>(data: T) -> Result<Self, Error>
    where
        T: Into<serde_json::Value>,
    {
        let value = data.into();
        let message = serde_json::from_value(value)?;

        Ok(message)
    }
}
