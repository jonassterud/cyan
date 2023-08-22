mod helpers;

use cyan_lib::prelude::*;

#[test]
fn test_serde() {
    let message = message::Message::EVENT {
        subscription_id: "https://example.com".to_string(),
        event: helpers::create_example_event().unwrap(),
    };

    let serialized = message.serialize().unwrap();
    let deserialized = message::Message::deserialize(serialized.clone()).unwrap();

    dbg!(serialized, deserialized);
}

#[test]
fn some_test() {
    let t = message::Filter::new().limit::<i64>(10);
    println!("{:?}", t.serialize());
}
