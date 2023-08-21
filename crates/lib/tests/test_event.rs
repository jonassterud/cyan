use anyhow::Result;
use cyan_lib::prelude::*;

fn create_example_event(secret_key: &str) -> Result<Event> {
    let secret_key = hex::decode(secret_key)?;
    let client = Client::from_secret_key(&secret_key)?;
    let event = client.create_event(1692452942, Kind::TextNote, vec![], "this is a test".to_string())?;

    Ok(event)
}

#[test]
fn create_event() {
    // Create the event
    let event = create_example_event("720f8a88fe12e0b6f635b7f6e085bf55175334a78b70f3a781478b428e347483").unwrap();

    // Check the id
    assert_eq!(hex::encode(event.id), "6af93de56bf823a19fb2c996e43f74186b09389ae3f663fa1ac96959060ca671");

    // Check the signature
    event.check_sig().unwrap();
}

#[test]
fn serialize_event() {
    // Create the event
    let event = create_example_event("720f8a88fe12e0b6f635b7f6e085bf55175334a78b70f3a781478b428e347483").unwrap();

    // Serialize
    let _serialized = event.serialize().unwrap();
    // dbg!(_serialized);
}

#[test]
fn deserialize_event() {
    let data = "{\"id\":\"6af93de56bf823a19fb2c996e43f74186b09389ae3f663fa1ac96959060ca671\",\"pubkey\":\"cd3a4037ff1ac2fbac7a89f5f6aaa08869b69caf10ea50ee50c258d9fdd00b19\",\"created_at\":1692452942,\"kind\":1,\"tags\":[],\"content\":\"this is a test\",\"sig\":\"e40d1f6b5ed0757392aa820a2e7577f1ad6ffcaf79e9f3c8c488ea1f10a44738dbe51742b45af68ad011b8703dae6430190037b4a0458333975a70a26dd4db5c\"}";

    // Deserialize the event
    let _event = Event::deserialize(data.as_bytes()).unwrap();
    // dbg!(_event);
}
