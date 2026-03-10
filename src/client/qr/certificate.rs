use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::prelude::Environment;
use base64::{Engine as _, engine::general_purpose};
use openssl::ecdsa::EcdsaSig;
use openssl::hash::{MessageDigest, hash};
use openssl::md::MdRef;
use openssl::pkey::PKey;
use openssl::pkey_ctx::PkeyCtx;
use openssl::rsa::Padding;
use openssl::sign::RsaPssSaltlen;
use openssl::sign::Signer;

fn pad_be(mut v: Vec<u8>, width: usize) -> Vec<u8> {
    if v.len() >= width {
        let start = v.len().saturating_sub(width);
        v.split_off(start)
    } else {
        let mut out = vec![0u8; width - v.len()];
        out.extend_from_slice(&v);
        out
    }
}

// Helper: convert MessageDigest -> &MdRef.
// The openssl 0.10 API expects `&MdRef` but we create a `MessageDigest`.
// To avoid scattering `unsafe` blocks, centralize the conversion here
fn md_ref_from_message_digest(md: MessageDigest) -> &'static MdRef {
    unsafe {
        let ptr = md.as_ptr();
        &*(ptr as *const MdRef)
    }
}

pub fn build_certificate_verification_url(
    client: &KsefClient,
    context_id_type: &str,
    context_id_value: &str,
    seller_nip: &str,
    cert_serial: &str,
    invoice_hash_b64url: &str,
    private_key_pem_opt: Option<&str>,
) -> Result<String, KsefError> {
    let host = client
        .environment
        .as_ref()
        .map(|e| match e {
            Environment::Test => "https://qr-test.ksef.mf.gov.pl",
            Environment::Prod => "https://qr.ksef.mf.gov.pl",
        })
        .unwrap_or("https://qr-test.ksef.mf.gov.pl")
        .trim_end_matches('/')
        .to_string();

    let ctx_type = context_id_type.trim();
    let ctx_value = context_id_value.trim();
    let nip = seller_nip.trim();
    let cert = cert_serial.trim();

    let normalized_hash = invoice_hash_b64url
        .trim()
        .replace('+', "-")
        .replace('/', "_")
        .trim_end_matches('=')
        .to_string();

    let path = format!(
        "/certificate/{}/{}/{}/{}/{}",
        ctx_type, ctx_value, nip, cert, normalized_hash
    );

    let unsigned_url = format!("{}{}", host, path);

    let private_pem = match private_key_pem_opt {
        Some(p) => p,
        None => return Ok(unsigned_url),
    };

    let host_no_proto = host
        .strip_prefix("https://")
        .or_else(|| host.strip_prefix("http://"))
        .unwrap_or(&host);

    let signing_input = format!("{}{}", host_no_proto.trim_end_matches('/'), path);
    let data = signing_input.as_bytes();

    let pkey = PKey::private_key_from_pem(private_pem.as_bytes())?;

    let sig_bytes = match pkey.id() {
        openssl::pkey::Id::RSA => {
            let mut ctx = PkeyCtx::new(&pkey)?;
            ctx.sign_init()?;
            ctx.set_rsa_padding(Padding::PKCS1_PSS)?;
            let md = MessageDigest::sha256();
            let md_ref = md_ref_from_message_digest(md);
            ctx.set_signature_md(md_ref)?;
            ctx.set_rsa_mgf1_md(md_ref)?;
            let digest = hash(MessageDigest::sha256(), data)?;
            ctx.set_rsa_pss_saltlen(RsaPssSaltlen::custom(32))?;
            let required = ctx.sign(&digest, None)?;
            let mut buf = vec![0u8; required as usize];
            let sig_len = ctx.sign(&digest, Some(&mut buf[..]))?;
            buf.truncate(sig_len);
            buf
        }
        openssl::pkey::Id::EC => {
            let mut signer = Signer::new(MessageDigest::sha256(), &pkey)?;
            signer.update(data)?;
            let der = signer.sign_to_vec()?;
            let ecsig = EcdsaSig::from_der(&der)?;
            let r = ecsig.r().to_vec();
            let s = ecsig.s().to_vec();
            let r_p = pad_be(r, 32);
            let s_p = pad_be(s, 32);
            [r_p, s_p].concat()
        }
        other => {
            return Err(KsefError::ApplicationError(
                0,
                format!("Unsupported key type for signing: {:?}", other),
            ));
        }
    };

    let signature_b64url = general_purpose::URL_SAFE_NO_PAD.encode(&sig_bytes);
    Ok(format!("{}/{}", unsigned_url, signature_b64url))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{ContextIdentifier, ContextIdentifierType, Environment, KsefClient};
    use openssl::ec::EcKey;
    use openssl::nid::Nid;
    use openssl::pkey::PKey;
    use openssl::rsa::Rsa;

    #[test]
    fn invoice_hash_normalization_and_unsigned_url() {
        let context = ContextIdentifier {
            id_type: ContextIdentifierType::Nip,
            value: "1111111111".to_string(),
        };
        let client = KsefClient::new(Environment::Test, context);

        let hash = "UtQp9Gpc51y+u3xApZjIjgkpZ01js/J8KflSPW8WzIE==";
        let url = build_certificate_verification_url(
            &client,
            "Nip",
            "1111111111",
            "1111111111",
            "01F20A5D352AE590",
            hash,
            None,
        )
        .expect("should produce unsigned url");
        assert!(url.starts_with("https://qr-test.ksef.mf.gov.pl/certificate/"));
        assert!(url.contains("UtQp9Gpc51y-u3xApZjIjgkpZ01js_J8KflSPW8WzIE"));
    }

    #[test]
    fn sign_with_ec_and_rsa_keys() {
        let ec_key =
            EcKey::generate(&openssl::ec::EcGroup::from_curve_name(Nid::X9_62_PRIME256V1).unwrap())
                .expect("ec gen");
        let pkey_ec = PKey::from_ec_key(ec_key).expect("pkey ec");
        let pem_ec = String::from_utf8(pkey_ec.private_key_to_pem_pkcs8().unwrap()).unwrap();

        let context = ContextIdentifier {
            id_type: ContextIdentifierType::Nip,
            value: "2222222222".to_string(),
        };
        let client = KsefClient::new(Environment::Test, context);

        let url_ec = build_certificate_verification_url(
            &client,
            "Nip",
            "2222222222",
            "2222222222",
            "SERIAL123",
            "abc123-_",
            Some(&pem_ec),
        )
        .expect("ec signed url");
        assert!(url_ec.contains("/SERIAL123/"));
        assert!(url_ec.rsplit('/').next().unwrap().len() > 10);

        let rsa = Rsa::generate(2048).expect("rsa gen");
        let pkey_rsa = PKey::from_rsa(rsa).expect("pkey rsa");
        let pem_rsa = String::from_utf8(pkey_rsa.private_key_to_pem_pkcs8().unwrap()).unwrap();

        let url_rsa = build_certificate_verification_url(
            &client,
            "Nip",
            "2222222222",
            "2222222222",
            "SERIAL456",
            "abc123-_",
            Some(&pem_rsa),
        )
        .expect("rsa signed url");
        assert!(url_rsa.contains("/SERIAL456/"));
        assert!(url_rsa.rsplit('/').next().unwrap().len() > 10);
    }
}
