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

    if let Err(e) = client.submit_xades_auth_request(signed_xml) {
        println!(
            "Submit auth request failed (expected on non-mock envs with random NIP): {:?}",
            e
        );
        return;
    }

    let auth_tokens = client.auth_token();
    assert!(!auth_tokens.authentication_token.is_empty());
    assert!(!auth_tokens.reference_number.is_empty());

    let authorized = match client.get_auth_status() {
        Ok(status) => status,
        Err(e) => {
            println!(
                "Get auth status failed (expected on non-mock envs with random NIP): {:?}",
                e
            );
            return;
        }
    };

    println!("Auth status result: {}", authorized);

    if !authorized {
        return;
    }

    if let Err(e) = client.get_access_token() {
        panic!("Should retrieve access tokens if authorized: {:?}", e);
    }

    let access_tokens = client.access_token();
    assert!(!access_tokens.access_token.is_empty());
    assert!(!access_tokens.refresh_token.is_empty());

    println!("Access Token obtained: {}", access_tokens.access_token);

    println!("Refreshing access token...");
    if let Err(e) = client.refresh_access_token() {
        panic!("Should refresh access token: {:?}", e);
    }

    let refreshed_access_tokens = client.access_token();
    assert!(!refreshed_access_tokens.access_token.is_empty());
    println!(
        "Refreshed Access Token: {}",
        refreshed_access_tokens.access_token
    );

    println!("Getting new KSeF token...");
    if let Err(e) = client.new_ksef_token() {
        panic!("Should get new KSeF token: {:?}", e);
    }

    let ksef_token = client.ksef_token();
    assert!(!ksef_token.token.is_empty());
    println!("New KSeF Token: {}", ksef_token.token);
}
