use anyhow::{anyhow, Result};
use cyan_lib::prelude::*;
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Clone)]
pub struct AppState(Arc<Mutex<InnerAppState>>);

impl AppState {
    /// Create a new app state.
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(InnerAppState::new())))
    }

    /// Get the inner app state.
    pub fn inner(&self) -> Result<MutexGuard<InnerAppState>> {
        Ok(self.0.lock().map_err(|_| anyhow!("failed locking"))?)
    }
}

pub struct InnerAppState {
    pub client: Client,
}

impl InnerAppState {
    /// Create a new inner app state.
    pub fn new() -> Self {
        Self { client: Client::new() }
    }
}
