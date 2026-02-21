use crate::client::error::KsefError;
use crate::client::ksef_certificates::get_enrollment_data::EnrollmentData;
use base64::Engine;
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::x509::{X509NameBuilder, X509ReqBuilder};

pub struct CsrResult {
    pub csr_base64: String,
    pub private_key_pem: String,
}

pub fn generate_csr(enrollment_data: &EnrollmentData) -> Result<CsrResult, KsefError> {
    let rsa = Rsa::generate(2048).map_err(|e| KsefError::RuntimeError(e.to_string()))?;
    let pkey = PKey::from_rsa(rsa).map_err(|e| KsefError::RuntimeError(e.to_string()))?;

    let mut x509_name =
        X509NameBuilder::new().map_err(|e| KsefError::RuntimeError(e.to_string()))?;

    x509_name
        .append_entry_by_text("CN", &enrollment_data.common_name)
        .map_err(|e| KsefError::RuntimeError(e.to_string()))?;

    x509_name
        .append_entry_by_text("C", &enrollment_data.country_name)
        .map_err(|e| KsefError::RuntimeError(e.to_string()))?;

    if let Some(gn) = &enrollment_data.given_name {
        x509_name
            .append_entry_by_text("givenName", gn)
            .map_err(|e| KsefError::RuntimeError(e.to_string()))?;
    }

    if let Some(sn) = &enrollment_data.surname {
        x509_name
            .append_entry_by_text("surname", sn)
            .map_err(|e| KsefError::RuntimeError(e.to_string()))?;
    }

    if let Some(serial) = &enrollment_data.serial_number {
        x509_name
            .append_entry_by_text("serialNumber", serial)
            .map_err(|e| KsefError::RuntimeError(e.to_string()))?;
    }

    if let Some(uid) = &enrollment_data.unique_identifier {
        x509_name
            .append_entry_by_text("uniqueIdentifier", uid)
            .map_err(|e| KsefError::RuntimeError(e.to_string()))?;
    }

    if let Some(org) = &enrollment_data.organization_name {
        x509_name
            .append_entry_by_text("O", org)
            .map_err(|e| KsefError::RuntimeError(e.to_string()))?;
    }

    if let Some(org_id) = &enrollment_data.organization_identifier {
        x509_name
            .append_entry_by_text("organizationIdentifier", org_id)
            .map_err(|e| KsefError::RuntimeError(e.to_string()))?;
    }

    let x509_name = x509_name.build();

    let mut req_builder =
        X509ReqBuilder::new().map_err(|e| KsefError::RuntimeError(e.to_string()))?;
    req_builder
        .set_pubkey(&pkey)
        .map_err(|e| KsefError::RuntimeError(e.to_string()))?;
    req_builder
        .set_subject_name(&x509_name)
        .map_err(|e| KsefError::RuntimeError(e.to_string()))?;

    req_builder
        .sign(&pkey, MessageDigest::sha256())
        .map_err(|e| KsefError::RuntimeError(e.to_string()))?;

    let req = req_builder.build();
    let der = req
        .to_der()
        .map_err(|e| KsefError::RuntimeError(e.to_string()))?;

    let csr_base64 = base64::engine::general_purpose::STANDARD.encode(&der);

    let private_key_pem = String::from_utf8(
        pkey.private_key_to_pem_pkcs8()
            .map_err(|e| KsefError::RuntimeError(e.to_string()))?,
    )
    .map_err(|e| KsefError::RuntimeError(e.to_string()))?;

    Ok(CsrResult {
        csr_base64,
        private_key_pem,
    })
}
