use crate::client::error::KsefError;

use openssl::pkcs12::ParsedPkcs12_2;
use openssl::pkcs12::Pkcs12;

pub mod gen_selfsign_cert;
pub mod sign;
pub mod utils;

pub struct XadesSigner {
    pkcs12: Option<ParsedPkcs12_2>,
}

impl Default for XadesSigner {
    fn default() -> Self {
        XadesSigner { pkcs12: None }
    }
}

impl XadesSigner {
    pub fn gen_selfsign_cert(
        &mut self,
        given_name: &str,
        surname: &str,
        serial_prefix: &str,
        nip: &str,
        common_name: &str,
    ) -> Result<(), KsefError> {
        let pkcs12 = gen_selfsign_cert::gen_selfsign_cert(
            given_name,
            surname,
            serial_prefix,
            nip,
            common_name,
        )?;
        self.pkcs12 = Some(pkcs12);
        Ok(())
    }

    pub fn load_pkcs12(&mut self, pkcs12_data: &[u8], password: &str) -> Result<(), KsefError> {
        if pkcs12_data.is_empty() {
            return Err(KsefError::Unexpected("Empty PKCS#12 data provided".into()));
        }

        let pkcs12 = Pkcs12::from_der(pkcs12_data).map_err(|e| {
            KsefError::Unexpected(format!("OpenSSL error while parsing PKCS#12 DER: {}", e))
        })?;

        let parsed = pkcs12.parse2(password).map_err(|e| {
            KsefError::Unexpected(format!(
                "Failed to parse PKCS#12 (likely wrong password or corrupted data): {}",
                e
            ))
        })?;

        self.pkcs12 = Some(parsed);
        Ok(())
    }

    pub fn sign(&self, xml: &str) -> Result<String, KsefError> {
        let pk = self
            .pkcs12
            .as_ref()
            .ok_or_else(|| KsefError::Unexpected("PKCS#12 certificate not generated".into()))?;
        sign::sign(xml, pk)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::xades::gen_selfsign_cert;
    use openssl::pkcs12::Pkcs12;

    #[test]
    fn load_pkcs12_valid() {
        let parsed =
            gen_selfsign_cert::gen_selfsign_cert("Jan", "Kowalski", "TST", "1234567890", "CN=Test")
                .expect("gen_selfsign_cert failed");
        let pkey = parsed.pkey.as_ref().expect("private key missing");
        let cert = parsed.cert.as_ref().expect("certificate missing");

        let pkcs12 = Pkcs12::builder()
            .name("test")
            .pkey(pkey)
            .cert(cert)
            .build2("")
            .expect("failed to build pkcs12");
        let der = pkcs12.to_der().expect("failed to serialize pkcs12");

        let mut signer = XadesSigner::default();
        signer
            .load_pkcs12(&der, "")
            .expect("should load valid pkcs12");
    }

    #[test]
    fn load_pkcs12_empty() {
        let mut signer = XadesSigner::default();
        let res = signer.load_pkcs12(&[], "");
        assert!(res.is_err(), "empty data should return error");
        match res {
            Err(KsefError::Unexpected(msg)) => assert!(msg.contains("Empty PKCS#12")),
            other => panic!("unexpected error type: {:?}", other),
        }
    }

    #[test]
    fn load_pkcs12_wrong_password() {
        let parsed =
            gen_selfsign_cert::gen_selfsign_cert("Jan", "Kowalski", "TST", "1234567890", "CN=Test")
                .expect("gen_selfsign_cert failed");
        let pkey = parsed.pkey.as_ref().expect("private key missing");
        let cert = parsed.cert.as_ref().expect("certificate missing");

        let pkcs12 = Pkcs12::builder()
            .name("test")
            .pkey(pkey)
            .cert(cert)
            .build2("correct")
            .expect("failed to build pkcs12 with password");
        let der = pkcs12.to_der().expect("failed to serialize pkcs12");

        let mut signer = XadesSigner::default();
        let res = signer.load_pkcs12(&der, "wrong");
        assert!(res.is_err(), "wrong password should fail to parse");
        match res {
            Err(KsefError::Unexpected(msg)) => assert!(msg.contains("Failed to parse PKCS#12")),
            other => panic!("unexpected error type: {:?}", other),
        }
    }
}
