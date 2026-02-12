use crate::client::error::KsefError;

use openssl::pkcs12::ParsedPkcs12_2;

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

    pub fn sign(&self, xml: &str) -> Result<String, KsefError> {
        let pk = self
            .pkcs12
            .as_ref()
            .ok_or_else(|| KsefError::Unexpected("PKCS#12 certificate not generated".into()))?;
        sign::sign(xml, pk)
    }
}
