//! Custom serialization / deserialization.
//!
//! This module is overly complicated to be honest.
//! It would be much easier if strings were used instead of arrays.
//! Maybe it's also possible to get rid of this whole module, and instead rely on
//! macros and such in order to deserialize/serialize.
//! Also, this could, maybe should, be split into different modules.

use super::{Event, Kind, Tag};

use arrayref::array_ref;
use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{self, Serialize, SerializeStruct, Serializer};
use std::fmt;

struct EventVisitor;
struct KindVisitor;
struct TagVisitor;

// Serialization of Event

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

// Deserialization of Event

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
        let mut tags: Option<Vec<Tag>> = None;
        let mut content: Option<String> = None;
        let mut sig: Option<String> = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "id" => id.replace(map.next_value::<String>()?).map_or_else(|| Ok(()), |_| Err(de::Error::duplicate_field("id")))?,
                "pubkey" => pubkey.replace(map.next_value::<String>()?).map_or_else(|| Ok(()), |_| Err(de::Error::duplicate_field("pubkey")))?,
                "created_at" => created_at.replace(map.next_value::<i64>()?).map_or_else(|| Ok(()), |_| Err(de::Error::duplicate_field("created_at")))?,
                "kind" => kind.replace(map.next_value::<i32>()?).map_or_else(|| Ok(()), |_| Err(de::Error::duplicate_field("kind")))?,
                "tags" => tags.replace(map.next_value::<Vec<Tag>>()?).map_or_else(|| Ok(()), |_| Err(de::Error::duplicate_field("tags")))?,
                "content" => content.replace(map.next_value::<String>()?).map_or_else(|| Ok(()), |_| Err(de::Error::duplicate_field("content")))?,
                "sig" => sig.replace(map.next_value::<String>()?).map_or_else(|| Ok(()), |_| Err(de::Error::duplicate_field("sig")))?,
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
            kind: Kind::try_from(kind.ok_or_else(|| de::Error::missing_field("kind"))?).map_err(de::Error::custom)?,
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

// Serialization of Kind

impl Serialize for Kind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32((*self) as i32)
    }
}

// Deserialization of Kind

impl<'de> Visitor<'de> for KindVisitor {
    type Value = Kind;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("int Kind")
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Kind::try_from(value).map_err(de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for Kind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_i32(KindVisitor)
    }
}

// Serialization of Tag

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

// Deserialization of Tag

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
