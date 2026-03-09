use ksef_client::*;
use crate::common;

use openssl::ec::EcKey;
use openssl::nid::Nid;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;

#[tokio::test]
async fn test_build_invoice_verification_url_normalization() {
    let client: ksef_client::KsefClient = common::authorize_client().await;

    let input_hash = "UtQp9Gpc51y+u3xApZjIjgkpZ01js/J8KflSPW8WzIE==";
    let url = client.build_invoice_verification_url("1111111111", "01-02-2026", input_hash);

    let expected_hash = "UtQp9Gpc51y-u3xApZjIjgkpZ01js_J8KflSPW8WzIE";
    let expected_prefix = format!(
        "https://qr-test.ksef.mf.gov.pl/invoice/1111111111/01-02-2026/{}",
        expected_hash
    );

    assert!(
        url.starts_with(&expected_prefix),
        "Invoice verification URL was not built as expected. got: {} expected prefix: {}",
        url,
        expected_prefix
    );
}

#[tokio::test]
async fn test_build_certificate_verification_url_unsigned_and_signed_ec() {
    let client: ksef_client::KsefClient = common::authorize_client().await;

    let unsigned = client
        .build_certificate_verification_url(
            "Nip",
            "1111111111",
            "1111111111",
            "01F20A5D352AE590",
            "UtQp9Gpc51y-u3xApZjIjgkpZ01js-J8KflSPW8WzIE",
            None,
        )
        .expect("unsigned url");

    assert!(
        unsigned.starts_with("https://qr-test.ksef.mf.gov.pl/certificate/Nip/1111111111/1111111111/01F20A5D352AE590/UtQp9Gpc51y-u3xApZjIjgkpZ01js-J8KflSPW8WzIE"),
        "Unsigned certificate URL has unexpected format: {}",
        unsigned
    );

    let ec_key =
        EcKey::generate(&openssl::ec::EcGroup::from_curve_name(Nid::X9_62_PRIME256V1).unwrap())
            .expect("ec gen");
    let pkey_ec = PKey::from_ec_key(ec_key).expect("pkey ec");
    let pem_ec = String::from_utf8(pkey_ec.private_key_to_pem_pkcs8().unwrap()).unwrap();

    let signed = client
        .build_certificate_verification_url(
            "Nip",
            "1111111111",
            "1111111111",
            "01F20A5D352AE590",
            "UtQp9Gpc51y-u3xApZjIjgkpZ01js-J8KflSPW8WzIE",
            Some(&pem_ec),
        )
        .expect("signed url ec");

    assert!(
        signed.contains("/certificate/Nip/1111111111/1111111111/01F20A5D352AE590/"),
        "Signed EC URL missing expected prefix: {}",
        signed
    );

    let sig_segment = signed.rsplit('/').next().expect("signature segment");
    assert!(
        sig_segment.len() > 10 && !sig_segment.contains('='),
        "EC signature looks invalid: {}",
        sig_segment
    );
}

#[tokio::test]
async fn test_build_certificate_verification_url_signed_rsa() {
    let client: ksef_client::KsefClient = common::authorize_client().await;

    let rsa = Rsa::generate(2048).expect("rsa gen");
    let pkey_rsa = PKey::from_rsa(rsa).expect("pkey rsa");
    let pem_rsa = String::from_utf8(pkey_rsa.private_key_to_pem_pkcs8().unwrap()).unwrap();

    let signed = client
        .build_certificate_verification_url(
            "Nip",
            "2222222222",
            "2222222222",
            "SERIAL456",
            "abc123-_",
            Some(&pem_rsa),
        )
        .expect("signed url rsa");

    assert!(
        signed.contains("/certificate/Nip/2222222222/2222222222/SERIAL456/"),
        "Signed RSA URL missing expected prefix: {}",
        signed
    );

    let sig_segment = signed.rsplit('/').next().expect("signature segment");
    assert!(
        sig_segment.len() > 10 && !sig_segment.contains('='),
        "RSA signature looks invalid: {}",
        sig_segment
    );
}
