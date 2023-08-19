use cyan_lib::prelude::*;

#[test]
fn create_event() {
    // Create the event
    let secret_key = hex::decode("720f8a88fe12e0b6f635b7f6e085bf55175334a78b70f3a781478b428e347483").unwrap();
    let client = Client::from_secret_key(&secret_key).unwrap();
    let event = client.create_event(1692452942, 1, vec![], "this is a test".to_string()).unwrap();

    // Check the id
    assert_eq!(hex::encode(event.id), "6af93de56bf823a19fb2c996e43f74186b09389ae3f663fa1ac96959060ca671");

    // Check the signature
    event.check_sig().unwrap();
}
