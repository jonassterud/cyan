//! Custom serialization / deserialization of [`Event`].
//!
//! This module is overly complicated to be honest.
//! It would be much easier if strings were used instead of arrays.
//! Maybe it's also possible to get rid of this whole module, and instead rely on
//! macros and such in order to deserialize/serialize.

use super::Event;
use arrayref::array_ref;
use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::fmt;

// Serialization

impl Serialize for Event {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Event", 7)?;
        state.serialize_field("id", &hex::encode(&self.id))?;
        state.serialize_field("pubkey", &hex::encode(&self.pubkey))?;
        state.serialize_field("created_at", &self.created_at)?;
        state.serialize_field("kind", &self.kind)?;
        state.serialize_field("tags", &self.tags)?;
        state.serialize_field("content", &self.content)?;
        state.serialize_field("sig", &hex::encode(&self.sig))?;

        state.end()
    }
}

// Deserialization

struct EventVisitor;

impl<'de> Visitor<'de> for EventVisitor {
    type Value = Event;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct Event")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut id: Option<String> = None;
        let mut pubkey: Option<String> = None;
        let mut created_at: Option<i64> = None;
        let mut kind: Option<i32> = None;
        let mut tags = None;
        let mut content: Option<String> = None;
        let mut sig: Option<String> = None;

        while let Some(key) = map.next_key()? {
            match key {
                "id" => id.replace(map.next_value()?).map_or_else(|| Ok(()), |_| Err(de::Error::duplicate_field("id")))?,
                "pubkey" => pubkey.replace(map.next_value()?).map_or_else(|| Ok(()), |_| Err(de::Error::duplicate_field("pubkey")))?,
                "created_at" => created_at.replace(map.next_value()?).map_or_else(|| Ok(()), |_| Err(de::Error::duplicate_field("created_at")))?,
                "kind" => kind.replace(map.next_value()?).map_or_else(|| Ok(()), |_| Err(de::Error::duplicate_field("kind")))?,
                "tags" => tags.replace(map.next_value()?).map_or_else(|| Ok(()), |_| Err(de::Error::duplicate_field("tags")))?,
                "content" => content.replace(map.next_value()?).map_or_else(|| Ok(()), |_| Err(de::Error::duplicate_field("content")))?,
                "sig" => sig.replace(map.next_value()?).map_or_else(|| Ok(()), |_| Err(de::Error::duplicate_field("sig")))?,
                _ => panic!(),
            }
        }

        fn decode_hex_in_range<T, E>(option: Option<T>, name: &'static str, range: std::ops::Range<usize>) -> Result<Vec<u8>, E>
        where
            T: AsRef<[u8]>,
            E: de::Error,
        {
            let value = option.ok_or_else(|| de::Error::missing_field(name))?;
            let decoded = hex::decode(value).map_err(de::Error::custom)?;
            let decoded_in_range = decoded.get(range).ok_or_else(|| de::Error::custom("out of range"))?;

            Ok(decoded_in_range.to_vec())
        }

        let decoded_id = decode_hex_in_range(id, "id", 0..32)?;
        let decoded_pubkey = decode_hex_in_range(pubkey, "pubkey", 0..32)?;
        let decoded_sig = decode_hex_in_range(sig, "sig", 0..64)?;

        Ok(Event {
            id: *array_ref![decoded_id, 0, 32],
            pubkey: *array_ref![decoded_pubkey, 0, 32],
            created_at: created_at.ok_or_else(|| de::Error::missing_field("created_at"))?,
            kind: kind.ok_or_else(|| de::Error::missing_field("kind"))?,
            tags: tags.ok_or_else(|| de::Error::missing_field("tags"))?,
            content: content.ok_or_else(|| de::Error::missing_field("content"))?,
            sig: *array_ref![decoded_sig, 0, 64],
        })
    }
}

impl<'de> Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FIELDS: &'static [&'static str] = &["id", "pubkey", "created_at", "kind", "tags", "content", "sig"];

        deserializer.deserialize_struct("Event", FIELDS, EventVisitor)
    }
}
