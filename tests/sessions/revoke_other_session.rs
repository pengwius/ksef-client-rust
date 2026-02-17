use crate::common;

use std::time::Duration;

use ksef_client::{ContextIdentifierType, KsefClient, SubjectIdentifierType};

fn authorize_with_nip(nip: &str) -> KsefClient {
    let mut client = KsefClient::new();
    let given_name = "Test";
    let surname = "User";
    let serial_prefix = "TINPL";
    let common_name = "Test User";

    let auth_token_request = client
        .get_auth_token_request(
            nip,
            ContextIdentifierType::Nip,
            SubjectIdentifierType::CertificateSubject,
        )
        .expect("Failed to get auth token request");

    let unsigned_xml = auth_token_request.to_xml();

    client
        .xades
        .gen_selfsign_cert(given_name, surname, serial_prefix, nip, common_name)
        .expect("Failed to generate self-signed certificate");

    let signed_xml = client
        .xades
        .sign(&unsigned_xml)
        .expect("Failed to sign XML");

    client
        .authenticate_by_xades_signature(signed_xml)
        .expect("Failed to submit XAdES auth request");

    match client.get_auth_status() {
        Ok(true) => {}
        Ok(false) => panic!("Authentication not successful"),
        Err(e) => panic!("Error checking auth status: {:?}", e),
    }

    std::thread::sleep(Duration::from_millis(300));

    client
}

#[test]
fn test_revoke_other_session_by_reference() {
    let nip = common::generate_random_nip();

    let client1 = authorize_with_nip(&nip);
    let client2 = authorize_with_nip(&nip);

    let ref1 = client1.auth_token().reference_number.clone();
    let ref2 = client2.auth_token().reference_number.clone();

    println!("Client1 session reference: {}", ref1);
    println!("Client2 session reference: {}", ref2);

    assert_ne!(
        ref1, ref2,
        "Expected two distinct session reference numbers"
    );

    client1
        .revoke_session(&ref2)
        .expect("client1 failed to revoke client2 session by reference");

    std::thread::sleep(Duration::from_millis(500));

    let resp = client1
        .get_active_sessions(None)
        .expect("Failed to list active sessions after revoke");
    let sessions = resp.items;

    let found = sessions.iter().find(|s| s.reference_number == ref2);

    if let Some(s) = found {
        println!(
            "Found session {} after revoke with status code {}",
            ref2, s.status.code
        );
        assert!(
            s.status.code == 425 || s.status.code >= 400,
            "Expected revoked or non-active status for the revoked session"
        );
    } else {
        println!(
            "Session {} not present after revoke (treated as revoked)",
            ref2
        );
    }
}
