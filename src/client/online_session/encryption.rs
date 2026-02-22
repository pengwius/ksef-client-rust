use crate::client::error::KsefError;
use openssl::encrypt::Encrypter;
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::rand::rand_bytes;
use openssl::rsa::Padding;
use openssl::sha::sha256;
use openssl::symm::{Cipher, encrypt};
use openssl::x509::X509;

use crate::client::KsefClient;
use crate::client::get_public_key_certificates;

pub struct EncryptionData {
    pub encrypted_symmetric_key: Vec<u8>,
    pub initialization_vector: Vec<u8>,
    pub symmetric_key: Vec<u8>,
}

pub fn generate_encryption_data(client: &KsefClient) -> Result<EncryptionData, KsefError> {
    let public_key_pem = match get_public_key(client) {
        Ok(key) => key,
        Err(e) => return Err(e),
    };

    let mut symmetric_key = vec![0u8; 32];
    rand_bytes(&mut symmetric_key).map_err(|e| {
        KsefError::ApplicationError(0, format!("Failed to generate symmetric key: {}", e))
    })?;

    let mut iv = vec![0u8; 16];
    rand_bytes(&mut iv)
        .map_err(|e| KsefError::ApplicationError(0, format!("Failed to generate IV: {}", e)))?;

    let public_key = PKey::public_key_from_pem(public_key_pem.as_bytes()).map_err(|e| {
        KsefError::ApplicationError(0, format!("Failed to parse public key: {}", e))
    })?;

    let mut encrypter = Encrypter::new(&public_key).map_err(|e| {
        KsefError::ApplicationError(0, format!("Failed to create encrypter: {}", e))
    })?;

    encrypter
        .set_rsa_padding(Padding::PKCS1_OAEP)
        .map_err(|e| KsefError::ApplicationError(0, format!("Failed to set padding: {}", e)))?;

    encrypter
        .set_rsa_oaep_md(MessageDigest::sha256())
        .map_err(|e| KsefError::ApplicationError(0, format!("Failed to set OAEP digest: {}", e)))?;

    encrypter
        .set_rsa_mgf1_md(MessageDigest::sha256())
        .map_err(|e| KsefError::ApplicationError(0, format!("Failed to set MGF1 digest: {}", e)))?;

    let buffer_len = encrypter.encrypt_len(&symmetric_key).map_err(|e| {
        KsefError::ApplicationError(0, format!("Failed to get encrypt length: {}", e))
    })?;

    let mut encrypted_symmetric_key = vec![0u8; buffer_len];
    let encrypted_len = encrypter
        .encrypt(&symmetric_key, &mut encrypted_symmetric_key)
        .map_err(|e| {
            KsefError::ApplicationError(0, format!("Failed to encrypt symmetric key: {}", e))
        })?;

    encrypted_symmetric_key.truncate(encrypted_len);

    Ok(EncryptionData {
        encrypted_symmetric_key,
        initialization_vector: iv,
        symmetric_key,
    })
}

fn get_public_key(client: &KsefClient) -> Result<String, KsefError> {
    let certs = client.get_public_key_certificates()?;
    let cert = certs
        .iter()
        .find(|c| {
            c.usage.contains(
                &get_public_key_certificates::PublicKeyCertificateUsage::SymmetricKeyEncryption,
            )
        })
        .or_else(|| certs.first())
        .ok_or_else(|| {
            KsefError::ApplicationError(0, "No suitable public key certificate found".to_string())
        })?;

    let pem = format!(
        "-----BEGIN CERTIFICATE-----\n{}\n-----END CERTIFICATE-----",
        &cert.certificate
    );
    let cert = X509::from_pem(pem.as_bytes()).map_err(|e| {
        KsefError::ApplicationError(0, format!("Failed to parse certificate PEM: {}", e))
    })?;
    let public_key = cert.public_key().map_err(|e| {
        KsefError::ApplicationError(0, format!("Failed to get public key from cert: {}", e))
    })?;
    let public_key_pem = public_key.public_key_to_pem().map_err(|e| {
        KsefError::ApplicationError(0, format!("Failed to convert public key to PEM: {}", e))
    })?;
    let public_key_pem_str = String::from_utf8(public_key_pem).map_err(|e| {
        KsefError::ApplicationError(
            0,
            format!("Failed to convert public key PEM to string: {}", e),
        )
    })?;

    Ok(public_key_pem_str)
}

pub fn encrypt_invoice(content: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, KsefError> {
    let cipher = Cipher::aes_256_cbc();
    encrypt(cipher, key, Some(iv), content)
        .map_err(|e| KsefError::ApplicationError(0, format!("Failed to encrypt invoice: {}", e)))
}

pub fn hash_invoice(content: &[u8]) -> Vec<u8> {
    sha256(content).to_vec()
}
