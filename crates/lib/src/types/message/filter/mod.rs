//! The event filter used in subscriptions.

mod traits;

use super::custom_serde::serialize_tags_in_filter;
use crate::prelude::*;
use serde::Serialize;
use traits::UpdateField;

/// Used to filter events.
#[serde_with::skip_serializing_none]
#[derive(Serialize)]
pub struct Filter {
    pub ids: Option<Vec<event::ID>>,
    pub authors: Option<Vec<event::PUBKEY>>,
    pub kinds: Option<Vec<event::KIND>>,
    #[serde(serialize_with = "serialize_tags_in_filter")]
    pub tags: Option<Vec<event::TAG>>,
    /*
    #[serde(rename = "#e")]
    pub e: Option<Vec<event::TAG>>,
    #[serde(rename = "#p")]
    pub p: Option<Vec<event::Tag>>,
     */
    pub since: Option<event::CREATED_AT>,
    pub until: Option<event::CREATED_AT>,
    pub limit: Option<i64>,
}

impl Filter {
    /// Create a new empty filter.
    pub fn new() -> Self {
        Self {
            ids: None,
            authors: None,
            kinds: None,
            tags: None,
            since: None,
            until: None,
            limit: None,
        }
    }

    /// Serialize as JSON.
    pub fn serialize(&self) -> Result<serde_json::Value, Error> {
        let json = serde_json::to_value(self)?;

        Ok(json)
    }

    pub fn ids<T>(mut self, new: Vec<event::ID>) -> Self {
        Self::update_field(&mut self.ids, new);
        self
    }

    pub fn authors<T>(mut self, new: Vec<event::PUBKEY>) -> Self {
        Self::update_field(&mut self.authors, new);
        self
    }

    pub fn kinds<T>(mut self, new: Vec<event::KIND>) -> Self {
        Self::update_field(&mut self.kinds, new);
        self
    }

    pub fn tags<T>(mut self, new: Vec<event::TAG>) -> Self {
        Self::update_field(&mut self.tags, new);
        self
    }

    pub fn since<T>(mut self, new: event::CREATED_AT) -> Self {
        Self::update_field(&mut self.since, new);
        self
    }

    pub fn until<T>(mut self, new: event::CREATED_AT) -> Self {
        Self::update_field(&mut self.until, new);
        self
    }

    pub fn limit<T>(mut self, new: i64) -> Self {
        Self::update_field(&mut self.limit, new);
        self
    }

    fn update_field<T: UpdateField>(existing: &mut Option<T>, new: T) {
        if let Some(existing_value) = existing {
            existing_value.update(new);
        } else {
            *existing = Some(new);
        }
    }
}
