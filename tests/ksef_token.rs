mod common;

use ksef_client::{ContextIdentifierType, KsefClient, SubjectIdentifierType};

#[test]
fn test_ksef_token_lifecycle() {
    let mut client = KsefClient::new();
    let nip = common::generate_random_nip();
    let given_name = "Eugeniusz";
    let surname = "Fakturowski";
    let serial_prefix = "TINPL";
    let common_name = "Eugeniusz Fakturowski";

    println!("Starting KSeF token test for NIP: {}", nip);

    let auth_token_request = client
        .get_auth_token_request(
            &nip,
            ContextIdentifierType::Nip,
            SubjectIdentifierType::CertificateSubject,
        )
        .expect("Failed to get auth token request");

    let unsigned_xml = auth_token_request.to_xml();

    client
        .xades
        .gen_selfsign_cert(given_name, surname, serial_prefix, &nip, common_name)
        .expect("Failed to generate self-signed certificate");

    let signed_xml = client
        .xades
        .sign(&unsigned_xml)
        .expect("Failed to sign XML");

    if let Err(e) = client.authenticate_by_xades_signature(signed_xml) {
        println!("Authentication failed (expected for random NIP): {:?}", e);
        return;
    }

    if let Err(_) = client.get_access_token() {
        println!("Could not retrieve access token immediately, skipping KSeF token tests.");
        return;
    }

    if client.access_token().access_token.is_empty() {
        println!("No access token available, skipping KSeF token tests.");
        return;
    }

    println!("Generating new KSeF token...");
    match client.new_ksef_token(true) {
        Ok(token) => {
            assert!(
                !token.token.is_empty(),
                "Generated token should not be empty"
            );
            assert!(
                !token.reference_number.is_empty(),
                "Reference number should not be empty"
            );
        }
        Err(e) => panic!("Failed to generate new KSeF token: {:?}", e),
    }

    let ksef_token = client.ksef_token.clone();

    println!("Listing KSeF tokens...");
    match client.get_ksef_tokens() {
        Ok(tokens) => {
            assert!(!tokens.is_empty(), "Should have at least one token");
            let found = tokens
                .iter()
                .any(|t| t.reference_number == ksef_token.reference_number);
            assert!(found, "Newly generated token should be in the list");
        }
        Err(e) => panic!("Failed to list KSeF tokens: {:?}", e),
    }

    println!("Getting KSeF token status...");
    match client.get_ksef_token_status(&ksef_token.reference_number) {
        Ok(status) => {
            assert_eq!(status.reference_number, ksef_token.reference_number);
        }
        Err(e) => panic!("Failed to get KSeF token status: {:?}", e),
    }

    println!("Authenticating using KSeF token...");
    client.ksef_token.context_type = Some(ContextIdentifierType::Nip);
    client.ksef_token.context_value = Some(nip.clone());

    match client.authenticate_by_ksef_token() {
        Ok(()) => {
            let auth_token = client.auth_token();
            assert!(!auth_token.authentication_token.is_empty());
            println!("Authentication with KSeF token successful");
        }
        Err(e) => panic!("Failed to authenticate with KSeF token: {:?}", e),
    }

    println!("Revoking KSeF token...");
    match client.revoke_ksef_token(&ksef_token.reference_number) {
        Ok(()) => {
            println!("KSeF token revoked");
        }
        Err(e) => panic!("Failed to revoke KSeF token: {:?}", e),
    }

    match client.get_ksef_token_status(&ksef_token.reference_number) {
        Ok(status) => {
            println!("Token status after revocation: {:?}", status.status);
        }
        Err(e) => println!("Failed to get status after revocation: {:?}", e),
    }
}
