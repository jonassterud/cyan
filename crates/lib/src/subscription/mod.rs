use crate::prelude::*;
use rand::{distributions::Alphanumeric, Rng};

pub type ID = String;

pub struct Subscription {
    pub id: ID,
    pub filters: Vec<message::Filter>,
}

impl Subscription {
    /// Create a new subscription.
    pub fn new() -> Self {
        Self {
            id: rand::thread_rng().sample_iter(Alphanumeric).take(64).map(char::from).collect(),
            filters: Vec::new(),
        }
    }

    /// Get this subscription as a request message.
    pub fn as_req(&self) -> message::Message {
        message::Message::REQ {
            subscription_id: self.id.clone(),
            filters: self.filters.clone(),
        }
    }
}
