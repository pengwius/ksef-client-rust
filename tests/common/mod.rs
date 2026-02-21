use ksef_client::{ContextIdentifierType, KsefClient, SubjectIdentifierType};
use rand::random_range;

pub fn generate_random_nip() -> String {
    loop {
        let mut digits: Vec<u8> = (0..9).map(|_| random_range(0..10) as u8).collect();
        // Use a valid Tax Office prefix (e.g. 526 for Warszawa-Mokotów) to pass validation
        digits[0] = 5;
        digits[1] = 2;
        digits[2] = 6;

        let weights = [6, 5, 7, 2, 3, 4, 5, 6, 7];
        let sum: u32 = digits
            .iter()
            .zip(weights.iter())
            .map(|(d, w)| (*d as u32) * (*w as u32))
            .sum();

        let checksum = sum % 11;
        if checksum != 10 {
            digits.push(checksum as u8);
            return digits.iter().map(|d| d.to_string()).collect();
        }
    }
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

    match client.get_auth_status() {
        Ok(true) => {}
        Ok(false) => {
            eprintln!("Authentication status check failed: Authentication not successful");
            panic!("Authentication not successful");
        }
        Err(e) => {
            panic!("Error checking auth status: {:?}", e);
        }
    }

    let _ = client.get_access_token();

    client
}
