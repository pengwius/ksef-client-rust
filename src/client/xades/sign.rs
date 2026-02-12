use crate::client::error::KsefError;
use crate::client::xades::utils::xml_escape;
use base64::{Engine as _, engine::general_purpose};
use chrono::{SecondsFormat, Utc};
use openssl::hash::{MessageDigest, hash};
use openssl::pkcs12::ParsedPkcs12_2;
use openssl::pkey::PKey;
use openssl::pkey::PKeyRef;
use openssl::sign::Signer;
use openssl::x509::X509;
use regex::Regex;

pub fn sign(xml: &str, pkcs12: &ParsedPkcs12_2) -> Result<String, KsefError> {
    let pkey: &PKey<openssl::pkey::Private> = match &pkcs12.pkey {
        Some(pk) => pk,
        None => {
            return Err(KsefError::Unexpected(
                "PKCS#12 does not contain a private key".to_string(),
            ));
        }
    };

    let cert: &X509 = match &pkcs12.cert {
        Some(c) => c,
        None => {
            return Err(KsefError::Unexpected(
                "PKCS#12 does not contain a certificate".to_string(),
            ));
        }
    };

    let cert_der = cert.to_der()?;
    let cert_b64 = general_purpose::STANDARD.encode(&cert_der);

    let signed_properties = build_signed_properties(&cert, cert_der)?;
    let signed_properties_c14n = canonicalize_signed_properties_in_qp(&signed_properties);
    let digest_signed_properties = sha256_b64(signed_properties_c14n.as_bytes());

    let doc_for_digest = remove_existing_signature(xml);
    let doc_c14n = canonicalize_common(&doc_for_digest, true);
    let digest_doc = sha256_b64(doc_c14n.as_bytes());

    let signed_info = build_signed_info(&digest_doc, &digest_signed_properties);
    let signed_info_c14n = canonicalize_inclusive(&signed_info);

    let signature_value = rsa_sha256_sign_b64(pkey.as_ref(), signed_info_c14n.as_bytes())?;

    let key_info = format!(
        "<KeyInfo><X509Data><X509Certificate>{}</X509Certificate></X509Data></KeyInfo>",
        cert_b64
    );

    let qualifying_props = compact_xml(&format!(
        r###"
        <Object>
            <xades:QualifyingProperties Target="#Signature" xmlns:xades="http://uri.etsi.org/01903/v1.3.2#" xmlns="http://www.w3.org/2000/09/xmldsig#">
                {}
            </xades:QualifyingProperties>
        </Object>"###,
        signed_properties
    ));

    let signature = compact_xml(&format!(
        r###"
        <Signature Id="Signature" xmlns="http://www.w3.org/2000/09/xmldsig#">
            {}
            <SignatureValue>{}</SignatureValue>
            {}{}
        </Signature>"###,
        signed_info, signature_value, key_info, qualifying_props
    ));

    let signed_xml = insert_signature_into_enveloped(xml, &signature)?;

    Ok(signed_xml)
}

fn insert_signature_into_enveloped(
    xml: &str,
    signature_fragment: &str,
) -> Result<String, KsefError> {
    let s = xml.trim();
    let body = if s.starts_with("<?xml") {
        if let Some(pos) = s.find("?>") {
            s[(pos + 2)..].trim()
        } else {
            s
        }
    } else {
        s
    };

    let root_name = if let Some(start) = body.find('<') {
        if let Some(end) = body[start + 1..]
            .find(|c: char| c == ' ' || c == '>' || c == '\t' || c == '\n' || c == '\r' || c == '/')
        {
            let name = &body[start + 1..start + 1 + end];
            name.to_string()
        } else {
            return Err(KsefError::Unexpected(
                "failed to determine root element name".to_string(),
            ));
        }
    } else {
        return Err(KsefError::Unexpected("invalid XML input".to_string()));
    };

    let closing_tag = format!("</{}>", root_name);
    if let Some(pos) = s.rfind(&closing_tag) {
        let mut out = String::with_capacity(s.len() + signature_fragment.len() + 64);
        out.push_str(&s[..pos]);
        out.push_str(signature_fragment);
        out.push_str(&s[pos..]);
        return Ok(out);
    }

    Err(KsefError::Unexpected(
        "failed to find closing root tag to insert Signature".to_string(),
    ))
}

fn build_signed_properties(cert: &X509, cert_der: Vec<u8>) -> Result<String, KsefError> {
    let cert_digest_b64 = sha256_b64(&cert_der);

    let issuer = cert
        .issuer_name()
        .entries()
        .map(|e| {
            format!(
                "{}={}",
                e.object().nid().short_name().unwrap_or(""),
                e.data()
                    .as_utf8()
                    .map(|s| s.to_string())
                    .unwrap_or_default()
            )
        })
        .collect::<Vec<_>>()
        .join(", ");

    let serial_bn = cert
        .serial_number()
        .to_bn()
        .and_then(|bn| bn.to_dec_str().map(|s| s.to_string()))
        .unwrap_or_else(|_| "0".to_string());

    let signed_properties = format!(
        r###"
        <xades:SignedProperties Id="SignedProperties">
            <xades:SignedSignatureProperties>
                <xades:SigningTime>{}</xades:SigningTime>
                <xades:SigningCertificate>
                    <xades:Cert>
                        <xades:CertDigest>
                            <DigestMethod Algorithm="http://www.w3.org/2001/04/xmlenc#sha256" xmlns="http://www.w3.org/2000/09/xmldsig#" />
                            <DigestValue xmlns="http://www.w3.org/2000/09/xmldsig#">{}</DigestValue>
                        </xades:CertDigest>
                        <xades:IssuerSerial>
                            <X509IssuerName xmlns="http://www.w3.org/2000/09/xmldsig#">{}</X509IssuerName>
                            <X509SerialNumber xmlns="http://www.w3.org/2000/09/xmldsig#">{}</X509SerialNumber>
                        </xades:IssuerSerial>
                    </xades:Cert>
                </xades:SigningCertificate>
            </xades:SignedSignatureProperties>
        </xades:SignedProperties>"###,
        Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
        cert_digest_b64,
        xml_escape(&issuer),
        xml_escape(&serial_bn)
    );

    Ok(compact_xml(&signed_properties))
}

fn build_signed_info(digest_doc_b64: &str, digest_signed_props_b64: &str) -> String {
    let si = format!(
        r###"
        <SignedInfo>
            <CanonicalizationMethod Algorithm="http://www.w3.org/TR/2001/REC-xml-c14n-20010315" />
            <SignatureMethod Algorithm="http://www.w3.org/2001/04/xmldsig-more#rsa-sha256" />
            <Reference URI="">
                <Transforms>
                    <Transform Algorithm="http://www.w3.org/2000/09/xmldsig#enveloped-signature" />
                    <Transform Algorithm="http://www.w3.org/2001/10/xml-exc-c14n#" />
                </Transforms>
                <DigestMethod Algorithm="http://www.w3.org/2001/04/xmlenc#sha256" />
                <DigestValue>{}</DigestValue>
            </Reference>
            <Reference URI="#SignedProperties" Type="http://uri.etsi.org/01903#SignedProperties">
                <Transforms>
                    <Transform Algorithm="http://www.w3.org/2001/10/xml-exc-c14n#" />
                </Transforms>
                <DigestMethod Algorithm="http://www.w3.org/2001/04/xmlenc#sha256" />
                <DigestValue>{}</DigestValue>
            </Reference>
        </SignedInfo>"###,
        digest_doc_b64, digest_signed_props_b64
    );

    compact_xml(&si)
}

fn canonicalize_signed_properties_in_qp(signed_props: &str) -> String {
    let injected = signed_props.replacen(
        "<xades:SignedProperties",
        "<xades:SignedProperties xmlns:xades=\"http://uri.etsi.org/01903/v1.3.2#\"",
        1,
    );
    canonicalize_common(&injected, true)
}

fn canonicalize_inclusive(s: &str) -> String {
    let signedinfo_re = Regex::new(r"(?s)<SignedInfo").unwrap();
    if signedinfo_re.is_match(s) {
        let injected = s.replacen(
            "<SignedInfo",
            "<SignedInfo xmlns=\"http://www.w3.org/2000/09/xmldsig#\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\"",
            1,
        );
        return canonicalize_common(&injected, false);
    }

    canonicalize_common(s, false)
}

fn canonicalize_common(s: &str, strip_unused: bool) -> String {
    let mut out = s.trim().to_string();
    if out.starts_with("<?xml") {
        if let Some(idx) = out.find("?>") {
            out = out[(idx + 2)..].trim().to_string();
        }
    }

    let tag_re =
        Regex::new(r#"<([^\s/>]+)((?:\s+[^\s=/>]+=(?:'[^']*'|"[^"]*"))*)\s*(/?)>"#).unwrap();
    let attr_re = Regex::new(r#"([^\s=/>]+)=(?:'([^']*)'|"([^"]*)")"#).unwrap();

    let normalized = tag_re.replace_all(&out, |caps: &regex::Captures| {
        let name = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let attrs_block = caps.get(2).map(|m| m.as_str()).unwrap_or("");
        let self_close = caps.get(3).map(|m| m.as_str()).unwrap_or("");

        let mut attrs: Vec<(String, String)> = Vec::new();
        for ac in attr_re.captures_iter(attrs_block) {
            let k = ac.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
            let v = ac
                .get(2)
                .or_else(|| ac.get(3))
                .map(|m| m.as_str())
                .unwrap_or("")
                .to_string();

            if strip_unused {
                if k == "xmlns:xsi" || k == "xmlns:xsd" {
                    continue;
                }
            }

            attrs.push((k, v));
        }

        attrs.sort_by(|a, b| {
            let a_ns = a.0.starts_with("xmlns");
            let b_ns = b.0.starts_with("xmlns");
            match (a_ns, b_ns) {
                (true, true) => a.0.cmp(&b.0),
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                (false, false) => a.0.cmp(&b.0),
            }
        });

        let mut buf = String::new();
        buf.push('<');
        buf.push_str(name);
        for (k, v) in attrs {
            buf.push(' ');
            buf.push_str(&k);
            buf.push_str("=\"");
            let escaped = v
                .replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('"', "&quot;");
            buf.push_str(&escaped);
            buf.push('"');
        }

        if !self_close.is_empty() {
            buf.push_str("></");
            buf.push_str(name);
            buf.push('>');
        } else {
            buf.push('>');
        }
        buf
    });

    normalized.to_string()
}

fn remove_existing_signature(xml: &str) -> String {
    if let Some(start) = xml.find("<Signature") {
        if let Some(end) = xml[start..].find("</Signature>") {
            let end_pos = start + end + "</Signature>".len();
            let mut cleaned = String::with_capacity(xml.len() - (end_pos - start));
            cleaned.push_str(&xml[..start]);
            cleaned.push_str(&xml[end_pos..]);
            return cleaned;
        }
    }
    xml.to_string()
}

fn rsa_sha256_sign_b64(
    pkey: &PKeyRef<openssl::pkey::Private>,
    data: &[u8],
) -> Result<String, KsefError> {
    let mut signer = Signer::new(MessageDigest::sha256(), pkey)?;
    signer.update(data)?;
    let sig = signer.sign_to_vec()?;
    Ok(general_purpose::STANDARD.encode(&sig))
}

fn sha256_b64(bytes: &[u8]) -> String {
    let digest = hash(MessageDigest::sha256(), bytes).expect("sha256");
    general_purpose::STANDARD.encode(digest)
}

fn compact_xml(s: &str) -> String {
    let mut out = s.replace('\r', "").replace('\t', "");
    let re_between = Regex::new(r">\s+<").unwrap();
    out = re_between.replace_all(&out, "><").to_string();
    out.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::xades::gen_selfsign_cert::gen_selfsign_cert;
    use base64::engine::general_purpose;
    use openssl::pkey::PKey;
    use openssl::rsa::Rsa;

    #[test]
    fn test_insert_signature_into_enveloped_basic() {
        let xml = r#"<?xml version="1.0"?><Root><Child/></Root>"#;
        let sig = "<Signature>SIG</Signature>";
        let out = insert_signature_into_enveloped(xml, sig).expect("insert failed");
        assert!(out.contains("<Signature>SIG</Signature>"));
        let pos_sig = out.find("<Signature>").unwrap();
        let pos_close = out.rfind("</Root>").unwrap();
        assert!(pos_sig < pos_close);
    }

    #[test]
    fn test_build_signed_properties_contains_expected() {
        let pkcs12 =
            gen_selfsign_cert("Jan", "Kowalski", "TST", "123", "CN=Test").expect("cert gen failed");
        let cert = pkcs12.cert.as_ref().expect("missing cert");
        let der = cert.to_der().expect("to_der failed");
        let sp = build_signed_properties(cert, der).expect("build_signed_properties failed");
        assert!(sp.contains("<xades:SignedProperties"));
        assert!(sp.contains("<DigestValue"));
    }

    #[test]
    fn test_build_signed_info_includes_digests() {
        let si = build_signed_info("DOC_DIGEST", "PROP_DIGEST");
        assert!(si.contains("DOC_DIGEST"));
        assert!(si.contains("PROP_DIGEST"));
        assert!(si.contains("<SignedInfo"));
    }

    #[test]
    fn test_canonicalize_signed_properties_in_qp_adds_ns() {
        let sp = r#"<xades:SignedProperties Id="SignedProperties"></xades:SignedProperties>"#;
        let out = canonicalize_signed_properties_in_qp(sp);
        assert!(out.contains("xmlns:xades=\"http://uri.etsi.org/01903/v1.3.2#\""));
    }

    #[test]
    fn test_canonicalize_inclusive_adds_namespaces() {
        let s = "<SignedInfo><Child/></SignedInfo>";
        let out = canonicalize_inclusive(s);
        assert!(out.contains("http://www.w3.org/2000/09/xmldsig#"));
        assert!(out.contains("<SignedInfo"));
    }

    #[test]
    fn test_canonicalize_common_strips_and_orders_attrs() {
        let input = r#"<root xmlns:xsi="u" xmlns:xsd="v" b="2" a='1'/>"#;
        let out = canonicalize_common(input, true);
        assert!(!out.contains("xmlns:xsi"));
        assert!(!out.contains("xmlns:xsd"));
        assert!(out.contains("a=\"1\""));
        assert!(out.contains("b=\"2\""));
        assert!(out.contains("</root>"));
    }

    #[test]
    fn test_remove_existing_signature() {
        let xml = r#"<Root><Signature>ABC</Signature><Child/></Root>"#;
        let cleaned = remove_existing_signature(xml);
        assert!(!cleaned.contains("<Signature>ABC</Signature>"));
        assert!(cleaned.contains("<Child/>"));
    }

    #[test]
    fn test_rsa_sha256_sign_b64_and_decode() {
        let rsa = Rsa::generate(2048).expect("rsa gen");
        let pkey = PKey::from_rsa(rsa).expect("pkey");
        let sig_b64 = rsa_sha256_sign_b64(pkey.as_ref(), b"hello world").expect("sign");
        let sig = general_purpose::STANDARD
            .decode(sig_b64)
            .expect("base64 decode");
        assert!(!sig.is_empty());
    }

    #[test]
    fn test_sha256_b64_of_empty() {
        let empty_b64 = sha256_b64(b"");
        assert_eq!(empty_b64, "47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFU=");
    }

    #[test]
    fn test_compact_xml_whitespace_between_tags() {
        let input = "<a>\n   <b> x </b>\n</a>";
        let out = compact_xml(input);
        assert_eq!(out, "<a><b> x </b></a>");
    }

    #[test]
    fn test_sign_integration_generates_signature() {
        let pkcs12 =
            gen_selfsign_cert("Jan", "Kowalski", "TST", "123", "CN=Test").expect("cert gen failed");
        let xml = r#"<Envelope><Data>hello</Data></Envelope>"#;
        let signed = sign(xml, &pkcs12).expect("sign failed");
        assert!(signed.contains("<Signature"));
        assert!(signed.contains("<Data>hello</Data>"));
    }
}
