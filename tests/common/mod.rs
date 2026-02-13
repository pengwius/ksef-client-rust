use ksef_client::{ContextIdentifierType, KsefClient, SubjectIdentifierType};
use rand::Rng;
use rand::distributions::Uniform;

pub fn generate_random_nip() -> String {
    let mut rng = rand::thread_rng();
    let digits = Uniform::from(0..10);
    (0..10).map(|_| rng.sample(&digits).to_string()).collect()
}

#[allow(dead_code)]
pub fn authorize_client() -> KsefClient {
    let mut client = KsefClient::new();
    let nip = generate_random_nip();
    let given_name = "Eugeniusz";
    let surname = "Fakturowski";
    let serial_prefix = "TINPL";
    let common_name = "Eugeniusz Fakturowski";

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

    match client.authenticate_by_xades_signature(signed_xml) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Authentication request submission failed: {:?}", e);
            panic!("Failed to authenticate: {:?}", e);
        }
    }

    let mut attempts = 0;
    loop {
        match client.get_auth_status() {
            Ok(true) => break,
            Ok(false) => {
                attempts += 1;
                if attempts > 10 {
                    panic!("Timeout waiting for auth status");
                }
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
            Err(e) => {
                eprintln!("Error checking auth status: {:?}", e);
                break;
            }
        }
    }

    let _ = client.get_access_token();

    client
}
