//! Custom serialization / deserialization of [`Message`].

use super::Message;
use serde::de::Visitor;
use serde::{de, ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

// Serialization

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_seq(None)?;

        match self {
            Message::EVENT { event } => {
                state.serialize_element("EVENT")?;
                state.serialize_element(event)?;
            }
            Message::REQ { subscription_id, filters } => {
                state.serialize_element("REQ")?;
                state.serialize_element(subscription_id)?;
                state.serialize_element(filters)?;
            }
            Message::CLOSE { subscription_id } => {
                state.serialize_element("CLOSE")?;
                state.serialize_element(subscription_id)?;
            }
            Message::OK { event_id, status, message } => {
                state.serialize_element("OK")?;
                state.serialize_element(event_id)?;
                state.serialize_element(status)?;
                state.serialize_element(message)?;
            }
            Message::EOSE { subscription_id } => {
                state.serialize_element("EOSE")?;
                state.serialize_element(subscription_id)?;
            }
            Message::NOTICE { message } => {
                state.serialize_element("NOTICE")?;
                state.serialize_element(message)?;
            }
        }

        state.end()
    }
}

// Deserialization

struct MessageVisitor;

impl<'de> Visitor<'de> for MessageVisitor {
    type Value = Message;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("sequence Message")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        if let Some(message_type) = seq.next_element::<String>()? {
            match message_type.as_str() {
                "EVENT" => Ok(Message::EVENT {
                    event: seq.next_element()?.ok_or(de::Error::missing_field("event"))?,
                }),
                "REQ" => Ok(Message::REQ {
                    subscription_id: seq.next_element()?.ok_or(de::Error::missing_field("subscription_id"))?,
                    filters: seq.next_element()?.ok_or(de::Error::missing_field("filters"))?,
                }),
                "CLOSE" => Ok(Message::CLOSE {
                    subscription_id: seq.next_element()?.ok_or(de::Error::missing_field("subscription_id"))?,
                }),
                "OK" => Ok(Message::OK {
                    event_id: seq.next_element()?.ok_or(de::Error::missing_field("event_id"))?,
                    status: seq.next_element()?.ok_or(de::Error::missing_field("status"))?,
                    message: seq.next_element()?.ok_or(de::Error::missing_field("message"))?,
                }),
                "EOSE" => Ok(Message::EOSE {
                    subscription_id: seq.next_element()?.ok_or(de::Error::missing_field("subscription_id"))?,
                }),
                "NOTICE" => Ok(Message::NOTICE {
                    message: seq.next_element()?.ok_or(de::Error::missing_field("message"))?,
                }),
                _ => Err(de::Error::custom("unknown message variant")),
            }
        } else {
            Err(de::Error::custom("missing message variant"))
        }
    }
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(MessageVisitor)
    }
}
