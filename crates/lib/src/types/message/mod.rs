//! The message object, and all of its children.

#![allow(non_camel_case_types)]

mod custom_serde;
mod filter;

use crate::prelude::*;

pub use filter::Filter;

/// The different messages a client can send or receive.
#[derive(Debug, Clone)]
pub enum Message {
    EVENT { subscription_id: subscription::ID, event: event::Event },
    REQ { subscription_id: subscription::ID, filters: Vec<Filter> },
    CLOSE { subscription_id: subscription::ID },
    OK { event_id: event::ID, status: bool, message: String },
    EOSE { subscription_id: subscription::ID },
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
