//! The event filter used in subscriptions.

mod traits;

use crate::prelude::*;
use traits::UpdateField;

/// Used to filter events.
#[derive(Debug, Clone)]
pub struct Filter {
    pub ids: Option<Vec<event::ID>>,
    pub authors: Option<Vec<event::PUBKEY>>,
    pub kinds: Option<Vec<event::KIND>>,
    pub tags: Option<Vec<event::TAG>>,
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

    pub fn ids(mut self, new: Vec<event::ID>) -> Self {
        Self::update_field(&mut self.ids, new);
        self
    }

    pub fn authors(mut self, new: Vec<event::PUBKEY>) -> Self {
        Self::update_field(&mut self.authors, new);
        self
    }

    pub fn kinds(mut self, new: Vec<event::KIND>) -> Self {
        Self::update_field(&mut self.kinds, new);
        self
    }

    pub fn tags(mut self, new: Vec<event::TAG>) -> Self {
        Self::update_field(&mut self.tags, new);
        self
    }

    pub fn since(mut self, new: event::CREATED_AT) -> Self {
        Self::update_field(&mut self.since, new);
        self
    }

    pub fn until(mut self, new: event::CREATED_AT) -> Self {
        Self::update_field(&mut self.until, new);
        self
    }

    pub fn limit(mut self, new: i64) -> Self {
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
