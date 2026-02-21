use crate::common;

use ksef_client::OpenOnlineSessionRequestBuilder;
use ksef_client::PublicKeyCertificateUsage;

#[test]
fn test_open_online_session() {
    let client = common::authorize_client();

    let certs = client
        .get_public_key_certificates()
        .expect("Failed to get public key certificates");

    let encryption_cert = certs
        .iter()
        .find(|c| {
            c.usage
                .contains(&PublicKeyCertificateUsage::SymmetricKeyEncryption)
        })
        .or_else(|| certs.first())
        .expect("No suitable certificate found for encryption");

    let encryption_data = client
        .generate_encryption_data_from_cert(&encryption_cert.certificate)
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
