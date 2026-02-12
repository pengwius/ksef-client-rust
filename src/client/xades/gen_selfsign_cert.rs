use crate::client::error::KsefError;
use openssl::asn1::{Asn1Integer, Asn1Time};
use openssl::bn::BigNum;
use openssl::hash::MessageDigest;
use openssl::pkcs12::ParsedPkcs12_2;
use openssl::pkcs12::Pkcs12;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::x509::extension::{
    AuthorityKeyIdentifier, BasicConstraints, ExtendedKeyUsage, KeyUsage, SubjectKeyIdentifier,
};
use openssl::x509::{X509, X509NameBuilder};

pub fn gen_selfsign_cert(
    given_name: &str,
    surname: &str,
    serial_prefix: &str,
    nip: &str,
    common_name: &str,
) -> Result<ParsedPkcs12_2, KsefError> {
    let friendly_name = format!("{} {} {}-{}", given_name, surname, serial_prefix, nip);

    let rsa = Rsa::generate(2048)?;
    let pkey = PKey::from_rsa(rsa)?;

    let mut builder = X509NameBuilder::new()?;
    builder.append_entry_by_text("C", "PL")?;
    builder.append_entry_by_text("givenName", given_name)?;
    builder.append_entry_by_text("surname", surname)?;
    builder.append_entry_by_text("serialNumber", &format!("{}-{}", serial_prefix, nip))?;
    builder.append_entry_by_text("CN", common_name)?;
    let name = builder.build();

    let mut builder = X509::builder()?;
    builder.set_version(2)?;

    let mut serial_bn = BigNum::new()?;
    serial_bn.rand(64, openssl::bn::MsbOption::MAYBE_ZERO, false)?;
    let serial_asn1 = Asn1Integer::from_bn(&serial_bn)?;
    builder.set_serial_number(&serial_asn1)?;

    builder.set_subject_name(&name)?;
    builder.set_issuer_name(&name)?;

    let nb = Asn1Time::days_from_now(0)?;
    let na = Asn1Time::days_from_now(365)?;
    builder.set_not_before(&nb)?;
    builder.set_not_after(&na)?;

    builder.set_pubkey(&pkey)?;

    builder.append_extension(BasicConstraints::new().critical().ca().build()?)?;
    builder.append_extension(
        KeyUsage::new()
            .critical()
            .digital_signature()
            .non_repudiation()
            .key_encipherment()
            .build()?,
    )?;
    builder.append_extension(ExtendedKeyUsage::new().client_auth().build()?)?;
    builder.append_extension(
        SubjectKeyIdentifier::new().build(&builder.x509v3_context(None, None))?,
    )?;
    builder.append_extension(
        AuthorityKeyIdentifier::new()
            .keyid(false)
            .issuer(false)
            .build(&builder.x509v3_context(None, None))?,
    )?;

    builder.sign(&pkey, MessageDigest::sha256())?;
    let x509 = builder.build();

    let pkcs12 = Pkcs12::builder()
        .name(&friendly_name)
        .pkey(&pkey)
        .cert(&x509)
        .build2("")?;

    let parsed = pkcs12.parse2("")?;

    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::gen_selfsign_cert;

    #[test]
    fn generates_pkcs12_with_key_and_cert() {
        let parsed = match gen_selfsign_cert("Jan", "Kowalski", "TST", "1234567890", "CN=Test") {
            Ok(p) => p,
            Err(e) => panic!("gen_selfsign_cert returned error: {}", e),
        };
        assert!(
            parsed.pkey.is_some(),
            "private key missing in parsed PKCS#12"
        );
        assert!(
            parsed.cert.is_some(),
            "certificate missing in parsed PKCS#12"
        );
    }
}
