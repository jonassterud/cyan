use anyhow::Result;
use cyan_lib::prelude::*;

pub fn create_example_event() -> Result<event::Event> {
    let secret_key_hex = "720f8a88fe12e0b6f635b7f6e085bf55175334a78b70f3a781478b428e347483";
    let secret_key = hex::decode(secret_key_hex)?;
    let client = Client::from_secret_key(&secret_key)?;
    let event = client.create_event(1692452942, event::Kind::TextNote, vec![], "this is a test".to_string())?;

    Ok(event)
}
