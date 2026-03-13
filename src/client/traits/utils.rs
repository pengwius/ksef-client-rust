use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::qr;

pub trait KsefUtils {
    fn url_for(&self, path: &str) -> String;

    fn build_invoice_verification_url(
        &self,
        seller_nip: &str,
        issue_date_ddmmrrrr: &str,
        invoice_hash_base64url: &str,
    ) -> String;

    fn build_certificate_verification_url(
        &self,
        context_id_type: &str,
        context_id_value: &str,
        seller_nip: &str,
        cert_serial: &str,
        invoice_hash_base64url: &str,
        private_key_pem_opt: Option<&str>,
    ) -> Result<String, KsefError>;
}

impl KsefUtils for KsefClient {
    fn url_for(&self, path: &str) -> String {
        format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }

    fn build_invoice_verification_url(
        &self,
        seller_nip: &str,
        issue_date_ddmmrrrr: &str,
        invoice_hash_base64url: &str,
    ) -> String {
        qr::invoice::build_invoice_verification_url(
            self,
            seller_nip,
            issue_date_ddmmrrrr,
            invoice_hash_base64url,
        )
    }

    fn build_certificate_verification_url(
        &self,
        context_id_type: &str,
        context_id_value: &str,
        seller_nip: &str,
        cert_serial: &str,
        invoice_hash_base64url: &str,
        private_key_pem_opt: Option<&str>,
    ) -> Result<String, KsefError> {
        qr::certificate::build_certificate_verification_url(
            self,
            context_id_type,
            context_id_value,
            seller_nip,
            cert_serial,
            invoice_hash_base64url,
            private_key_pem_opt,
        )
    }
}
