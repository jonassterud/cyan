//! The message object, and all of its children.

#![allow(non_camel_case_types)]

mod custom_serde;
mod filter;

use crate::prelude::*;

pub use filter::Filter;
pub type SUBSCRIPTION_ID = String;

/// The different messages a client can send or receive.
#[derive(Debug)]
pub enum Message {
    EVENT { subscription_id: SUBSCRIPTION_ID, event: event::Event },
    REQ { subscription_id: SUBSCRIPTION_ID, filters: String },
    CLOSE { subscription_id: SUBSCRIPTION_ID },
    OK { event_id: event::ID, status: bool, message: String },
    EOSE { subscription_id: SUBSCRIPTION_ID },
    NOTICE { message: String },
}

impl Message {
    /// Serialize as JSON.
    pub fn serialize(&self) -> Result<serde_json::Value, Error> {
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
