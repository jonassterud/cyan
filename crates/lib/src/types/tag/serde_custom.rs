//! Custom serialization / deserialization of [`Tag`].
//!
//! Maybe it's possible to represents tags in another way.
//! This module might not be needed.

use super::Tag;
use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{self, Serialize, Serializer};
use std::fmt;

// Serialization

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

// Deserialization

struct TagVisitor;

impl<'de> Visitor<'de> for TagVisitor {
    type Value = Tag;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("sequence Tag")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let variant: &str = seq.next_element()?.ok_or_else(|| de::Error::custom("missing tag variant"))?;

        match variant {
            _ => {
                let mut data = String::new();

                while let Ok(Some(next_value)) = seq.next_element::<&str>() {
                    data += next_value;
                }

                Ok(Tag::Other { data })
            }
        }
    }
}

impl<'de> Deserialize<'de> for Tag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(TagVisitor)
    }
}
