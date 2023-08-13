//! Various helpful functions.

use crate::error::Error;
use std::time::SystemTime;

/// Get Unix timestamp in seconds.
pub fn get_unix_timestamp() -> Result<i64, Error> {
    let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs().try_into()?;

    Ok(timestamp)
}
