use crate::client::KsefClient;
use crate::prelude::Environment;

pub fn build_invoice_verification_url(
    client: &KsefClient,
    seller_nip: &str,
    issue_date_ddmmrrrr: &str,
    invoice_hash_b64url: &str,
) -> String {
    let host = client
        .environment
        .as_ref()
        .map(|e| match e {
            Environment::Test => "https://qr-test.ksef.mf.gov.pl",
            Environment::Demo => "https://qr-demo.ksef.mf.gov.pl",
            Environment::Prod => "https://qr.ksef.mf.gov.pl",
        })
        .unwrap_or("https://qr-test.ksef.mf.gov.pl");

    let nip = seller_nip.trim();
    let date = issue_date_ddmmrrrr.trim();
    let hash_input = invoice_hash_b64url.trim();

    let mut normalized_hash = String::with_capacity(hash_input.len());
    for ch in hash_input.chars() {
        match ch {
            '+' => normalized_hash.push('-'),
            '/' => normalized_hash.push('_'),
            '=' => {
                // do nothing
            }
            other => normalized_hash.push(other),
        }
    }

    let host_trimmed = host.trim_end_matches('/');
    let seg_invoice = "invoice";
    let nip_seg = nip.trim_matches('/');
    let date_seg = date.trim_matches('/');
    let hash_seg = normalized_hash.trim_matches('/');

    format!(
        "{}/{}/{}/{}/{}",
        host_trimmed, seg_invoice, nip_seg, date_seg, hash_seg
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{ContextIdentifier, ContextIdentifierType};

    #[test]
    fn test_normalize_hash_and_build_url_test_env() {
        let context = ContextIdentifier {
            id_type: ContextIdentifierType::Nip,
            value: "1111111111".to_string(),
        };
        let client = KsefClient::new(Environment::Test, context);

        let input_hash = "UtQp9Gpc51y+u3xApZjIjgkpZ01js/J8KflSPW8WzIE==";
        let url = build_invoice_verification_url(&client, "1111111111", "01-02-2026", input_hash);

        let expected_hash = "UtQp9Gpc51y-u3xApZjIjgkpZ01js_J8KflSPW8WzIE";
        let expected = format!(
            "https://qr-test.ksef.mf.gov.pl/invoice/1111111111/01-02-2026/{}",
            expected_hash
        );
        assert_eq!(url, expected);
    }

    #[test]
    fn test_build_url_prod_env_and_trim_inputs() {
        let context = ContextIdentifier {
            id_type: ContextIdentifierType::Nip,
            value: "2222222222".to_string(),
        };
        let client = KsefClient::new(Environment::Prod, context);

        let input_hash = "abc_def-ghi";
        let url =
            build_invoice_verification_url(&client, " 3333333333 ", " 02-03-2024 ", input_hash);

        let expected =
            "https://qr.ksef.mf.gov.pl/invoice/3333333333/02-03-2024/abc_def-ghi".to_string();
        assert_eq!(url, expected);
    }
}
