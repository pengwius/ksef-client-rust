use ksef_client::*;
mod common;

use ksef_client::{
    ContextIdentifier, ContextIdentifierType, Environment, KsefClient, SubjectIdentifierType,
};

#[tokio::test]
async fn test_auth_token_request_generation() {
    let nip: String = common::generate_random_nip().await;
    let context = ContextIdentifier {
        id_type: ContextIdentifierType::Nip,
        value: nip.clone(),
    };
    let client = KsefClient::new(Environment::Test, context);

    let auth_token_request = client
        .get_auth_token_request(SubjectIdentifierType::CertificateSubject)
        .await
        .expect("Failed to get auth token request");

    let xml = auth_token_request.to_xml();
    assert!(!xml.is_empty());
    assert!(xml.contains(&nip));
}

#[tokio::test]
async fn test_xades_signature_generation() {
    let nip: String = common::generate_random_nip().await;
    let context = ContextIdentifier {
        id_type: ContextIdentifierType::Nip,
        value: nip.clone(),
    };
    let mut client = KsefClient::new(Environment::Test, context);

    let auth_token_request = client
        .get_auth_token_request(SubjectIdentifierType::CertificateSubject)
        .await
        .expect("Failed to get auth token request");

    let unsigned_xml = auth_token_request.to_xml();

    client
        .xades
        .gen_selfsign_cert("Jan", "Kowalski", "TINPL", &nip, "Jan Kowalski")
        .expect("Failed to generate certificate");

    let signed_xml = client
        .xades
        .sign(&unsigned_xml)
        .expect("Failed to sign XML");

    assert!(signed_xml.contains("Signature"));
    assert!(signed_xml.contains("xades:SignedProperties"));
}

#[tokio::test]
async fn test_authentication_submission() {
    let nip: String = common::generate_random_nip().await;
    let context = ContextIdentifier {
        id_type: ContextIdentifierType::Nip,
        value: nip.clone(),
    };
    let mut client = KsefClient::new(Environment::Test, context);

    let auth_token_request = client
        .get_auth_token_request(SubjectIdentifierType::CertificateSubject)
        .await
        .expect("Failed to get auth token request");

    let unsigned_xml = auth_token_request.to_xml();

    client
        .xades
        .gen_selfsign_cert("Jan", "Kowalski", "TINPL", &nip, "Jan Kowalski")
        .expect("Failed to generate certificate");

    let signed_xml = client
        .xades
        .sign(&unsigned_xml)
        .expect("Failed to sign XML");

    match client.authenticate_by_xades_signature(signed_xml).await {
        Ok(_) => {
            let auth_token = client.auth_token();
            assert!(!auth_token.authentication_token.is_empty());
            assert!(!auth_token.reference_number.is_empty());
        }
        Err(e) => {
            panic!(
                "Authentication submission failed (expected for random NIP): {:?}",
                e
            );
        }
    }
}
