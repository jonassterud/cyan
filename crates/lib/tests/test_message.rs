mod helpers;

use cyan_lib::prelude::*;
use serde_json::json;

#[test]
fn test_serde() {
    let message = Message::EVENT {
        subscription_id: "https://example.com".to_string(),
        event: helpers::create_example_event().unwrap(),
    };

    let serialized = message.serialize().unwrap();
    let deserialized = Message::deserialize(serialized.clone()).unwrap();

    dbg!(serialized, deserialized);
}

#[test]
fn serialize_message() {}

#[test]
fn deserialize_message() {
    // Create the data
    let data = json!([
        "EVENT",
        "https://example.com",
        helpers::create_example_event().unwrap()
    ]);

    // Deserialize
    let _message = Message::deserialize(data).unwrap();
    //dbg!("{:?}", _message);
}
