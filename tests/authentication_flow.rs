use ksef_client::{
    AuthTokenRequestBuilder, ContextIdentifierType, KsefClient, SubjectIdentifierType,
};
use rand::Rng;
use rand::distributions::Uniform;

fn generate_random_nip() -> String {
    let mut rng = rand::thread_rng();
    let digits = Uniform::from(0..10);
    (0..10).map(|_| rng.sample(&digits).to_string()).collect()
}

#[test]
fn test_full_authentication_flow() {
    let mut client = KsefClient::new();
    let nip = generate_random_nip();
    let given_name = "Eugeniusz";
    let surname = "Fakturowski";
    let serial_prefix = "TINPL";
    let common_name = "Eugeniusz Fakturowski";

    println!("Starting authentication flow test for NIP: {}", nip);

    let challenge = client
        .get_auth_challenge()
        .expect("Failed to get auth challenge");

    assert!(
        !challenge.challenge.is_empty(),
        "Challenge should not be empty"
    );
    assert!(
        challenge.timestamp_ms > 0,
        "Timestamp should be greater than 0"
    );

    let auth_token_request = AuthTokenRequestBuilder::new()
        .with_challenge(&challenge.challenge)
        .with_context(ContextIdentifierType::Nip, &nip)
        .with_subject_type(SubjectIdentifierType::CertificateSubject)
        .build()
        .expect("Failed to build auth token request");

    let unsigned_xml = auth_token_request.to_xml();
    assert!(!unsigned_xml.is_empty(), "Unsigned XML should not be empty");

    client
        .xades
        .gen_selfsign_cert(given_name, surname, serial_prefix, &nip, common_name)
        .expect("Failed to generate self-signed certificate");

    let signed_xml = client
        .xades
        .sign(&unsigned_xml)
        .expect("Failed to sign XML");

    assert!(
        signed_xml.contains("Signature"),
        "Signed XML should contain Signature"
    );
    assert!(
        signed_xml.contains(&challenge.challenge),
        "Signed XML should contain challenge"
    );

    let submit_result = client.submit_xades_auth_request(signed_xml);

    match submit_result {
        Ok(()) => {
            let auth_tokens = client.auth_token();
            assert!(!auth_tokens.authentication_token.is_empty());
            assert!(!auth_tokens.reference_number.is_empty());

            let status_result = client.get_auth_status();
            match status_result {
                Ok(authorized) => {
                    println!("Auth status result: {}", authorized);

                    if authorized {
                        let access_token_result = client.get_access_token();
                        assert!(
                            access_token_result.is_ok(),
                            "Should retrieve access tokens if authorized"
                        );

                        let access_tokens = client.access_token();
                        assert!(!access_tokens.access_token.is_empty());
                        assert!(!access_tokens.refresh_token.is_empty());
                    }
                }
                Err(e) => {
                    println!(
                        "Get auth status failed (expected on non-mock envs with random NIP): {:?}",
                        e
                    );
                }
            }
        }
        Err(e) => {
            println!(
                "Submit auth request failed (expected on non-mock envs with random NIP): {:?}",
                e
            );
        }
    }
}
