mod helpers;

use cyan_lib::prelude::*;

#[test]
fn create_event() {
    let event = helpers::create_example_event().unwrap();

    assert_eq!(hex::encode(event.id), "6af93de56bf823a19fb2c996e43f74186b09389ae3f663fa1ac96959060ca671");
    event.check_sig().unwrap();
}

#[test]
fn test_serde() {
    let event = helpers::create_example_event().unwrap();

    let serialized = event.serialize().unwrap();
    let deserialized = event::Event::deserialize(serialized.clone()).unwrap();

    dbg!(serialized, deserialized);
}
