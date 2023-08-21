//! Custom serialization / deserialization of [`Kind`].

use super::Kind;
use std::fmt;
use serde::de::Visitor;
use serde::{Serialize, Serializer, de, Deserialize, Deserializer};

// Serialization

impl Serialize for Kind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32((*self) as i32)
    }
}

// Deserialization

struct KindVisitor;

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
