mod common;

use ksef_client::OpenOnlineSessionRequestBuilder;

#[test]
fn test_open_online_session() {
    let client = common::authorize_client();

    let encryption_data = client
        .generate_encryption_data()
        .expect("Failed to generate encryption data");

    let request = OpenOnlineSessionRequestBuilder::new()
        .with_encryption(
            &encryption_data.encrypted_symmetric_key,
            &encryption_data.initialization_vector,
        )
        .build()
        .expect("Failed to build OpenOnlineSessionRequest");

    let response = client
        .open_online_session(request)
        .expect("Failed to open online session");

    println!("Opened online session: {:?}", response);

    assert!(
        !response.reference_number.is_empty(),
        "Session reference number should not be empty"
    );
}
