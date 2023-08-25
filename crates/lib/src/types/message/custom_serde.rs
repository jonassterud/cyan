//! Custom serialization / deserialization.

use super::{Filter, Message};
use crate::prelude::*;

use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, SerializeSeq, SerializeStruct, Serializer};
use std::fmt;

struct MessageVisitor;

// Serialization of Message

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_seq(None)?;

        match self {
            Message::EVENT { subscription_id, event } => {
                state.serialize_element("EVENT")?;
                state.serialize_element(subscription_id)?;
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

// Deserialization of Message

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
                    subscription_id: seq.next_element::<String>()?.ok_or(de::Error::missing_field("subscription_id"))?,
                    event: event::Event::deserialize(seq.next_element::<serde_json::Value>()?.ok_or(de::Error::missing_field("event"))?).map_err(de::Error::custom)?,
                }),
                "REQ" => {
                    todo!("deserialize filters");
                    /*
                        let subscription_id = seq.next_element()?.ok_or(de::Error::missing_field("subscription_id"))?;
                        let mut filters = Vec::new();

                        while let Some(filter) = seq.next_element()? {
                            filters.push(filter)
                        }

                        Ok(Message::REQ { subscription_id, filters })
                    */
                }
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

// Serialization of Filter

impl Serialize for Filter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Filter", 7)?;

        fn serialize_if_some<S, V>(state: &mut S, k: &'static str, v: Option<&V>) -> Result<(), S::Error>
        where
            S: serde::ser::SerializeStruct,
            V: Serialize,
        {
            if let Some(v) = v {
                state.serialize_field(k, v)?;
            }

            Ok(())
        }

        fn serialize_tags_if_some<S>(state: &mut S, tags: Option<&Vec<event::Tag>>) -> Result<(), S::Error>
        where
            S: serde::ser::SerializeStruct,
        {
            if let Some(tags) = tags {
                let mut e_tags = Vec::new();
                let mut p_tags = Vec::new();

                for tag in tags {
                    match tag {
                        event::Tag::E { event_id, .. } => e_tags.push(event_id),
                        event::Tag::P { pubkey, .. } => p_tags.push(pubkey),
                        _ => panic!("can't serialize tag"),
                    }
                }

                if !e_tags.is_empty() {
                    state.serialize_field("#e", &e_tags)?;
                }

                if !p_tags.is_empty() {
                    state.serialize_field("#p", &p_tags)?;
                }
            }

            Ok(())
        }

        serialize_if_some(&mut state, "ids", self.ids.as_ref())?;
        serialize_if_some(&mut state, "authors", self.authors.as_ref())?;
        serialize_if_some(&mut state, "kinds", self.kinds.as_ref())?;
        serialize_tags_if_some(&mut state, self.tags.as_ref())?;
        serialize_if_some(&mut state, "since", self.since.as_ref())?;
        serialize_if_some(&mut state, "until", self.until.as_ref())?;
        serialize_if_some(&mut state, "limit", self.limit.as_ref())?;

        state.end()
    }
}
