use crate::client::xades::utils::xml_escape;

#[derive(Debug, Clone)]
pub enum ContextIdentifierType {
    Nip,
    InternalId,
    NipVatUe,
}

#[derive(Debug, Clone)]
pub enum SubjectIdentifierType {
    CertificateSubject,
    CertificateFingerprint,
}

impl SubjectIdentifierType {
    fn as_str(&self) -> &'static str {
        match self {
            SubjectIdentifierType::CertificateSubject => "certificateSubject",
            SubjectIdentifierType::CertificateFingerprint => "certificateFingerprint",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AuthenticationTokenAllowedIps {
    pub ip4_addresses: Vec<String>,
    pub ip4_masks: Vec<String>,
    pub ip4_ranges: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AuthenticationTokenAuthorizationPolicy {
    pub allowed_ips: AuthenticationTokenAllowedIps,
}

#[derive(Debug, Clone)]
pub struct AuthTokenRequest {
    pub challenge: String,
    pub context_type: ContextIdentifierType,
    pub context_value: String,
    pub subject_identifier_type: SubjectIdentifierType,
    pub certificate_fingerprint: Option<String>,
    pub authorization_policy: Option<AuthenticationTokenAuthorizationPolicy>,
}

impl AuthTokenRequest {
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();

        xml.push_str(r#"<?xml version="1.0" encoding="utf-8"?>"#);
        xml.push_str(r#"<AuthTokenRequest xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns="http://ksef.mf.gov.pl/auth/token/2.0">"#);

        xml.push_str("<Challenge>");
        xml.push_str(&xml_escape(&self.challenge));
        xml.push_str("</Challenge>");

        xml.push_str("<ContextIdentifier>");
        match self.context_type {
            ContextIdentifierType::Nip => {
                xml.push_str("<Nip>");
                xml.push_str(&xml_escape(&self.context_value));
                xml.push_str("</Nip>");
            }
            ContextIdentifierType::InternalId => {
                xml.push_str("<InternalId>");
                xml.push_str(&xml_escape(&self.context_value));
                xml.push_str("</InternalId>");
            }
            ContextIdentifierType::NipVatUe => {
                xml.push_str("<NipVatUe>");
                xml.push_str(&xml_escape(&self.context_value));
                xml.push_str("</NipVatUe>");
            }
        }
        xml.push_str("</ContextIdentifier>");

        xml.push_str("<SubjectIdentifierType>");
        xml.push_str(&xml_escape(self.subject_identifier_type.as_str()));
        xml.push_str("</SubjectIdentifierType>");

        if let Some(fp) = &self.certificate_fingerprint {
            xml.push_str("<CertificateFingerprint>");
            xml.push_str(&xml_escape(fp));
            xml.push_str("</CertificateFingerprint>");
        }

        if let Some(policy) = &self.authorization_policy {
            xml.push_str("<AuthorizationPolicy>");
            xml.push_str("<AllowedIps>");

            if !policy.allowed_ips.ip4_addresses.is_empty() {
                xml.push_str("<Ip4Addresses>");
                for ip in &policy.allowed_ips.ip4_addresses {
                    xml.push_str("<Ip4Address>");
                    xml.push_str(&xml_escape(ip));
                    xml.push_str("</Ip4Address>");
                }
                xml.push_str("</Ip4Addresses>");
            }

            if !policy.allowed_ips.ip4_masks.is_empty() {
                xml.push_str("<Ip4Masks>");
                for mask in &policy.allowed_ips.ip4_masks {
                    xml.push_str("<Ip4Mask>");
                    xml.push_str(&xml_escape(mask));
                    xml.push_str("</Ip4Mask>");
                }
                xml.push_str("</Ip4Masks>");
            }

            if !policy.allowed_ips.ip4_ranges.is_empty() {
                xml.push_str("<Ip4Ranges>");
                for range in &policy.allowed_ips.ip4_ranges {
                    xml.push_str("<Ip4Range>");
                    xml.push_str(&xml_escape(range));
                    xml.push_str("</Ip4Range>");
                }
                xml.push_str("</Ip4Ranges>");
            }

            xml.push_str("</AllowedIps>");
            xml.push_str("</AuthorizationPolicy>");
        }

        xml.push_str("</AuthTokenRequest>");
        xml
    }
}

#[derive(Debug, Default)]
pub struct AuthTokenRequestBuilder {
    challenge: Option<String>,
    context_type: Option<ContextIdentifierType>,
    context_value: Option<String>,
    subject_identifier_type: Option<SubjectIdentifierType>,
    certificate_fingerprint: Option<String>,
    authorization_policy: Option<AuthenticationTokenAuthorizationPolicy>,
}

impl AuthTokenRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_challenge(mut self, challenge: impl Into<String>) -> Self {
        self.challenge = Some(challenge.into());
        self
    }

    pub fn with_context(
        mut self,
        ctx_type: ContextIdentifierType,
        ctx_value: impl Into<String>,
    ) -> Self {
        self.context_type = Some(ctx_type);
        self.context_value = Some(ctx_value.into());
        self
    }

    pub fn with_subject_type(mut self, subject_type: SubjectIdentifierType) -> Self {
        self.subject_identifier_type = Some(subject_type);
        self
    }

    pub fn with_certificate_fingerprint(mut self, fingerprint: impl Into<String>) -> Self {
        self.certificate_fingerprint = Some(fingerprint.into());
        self
    }

    pub fn with_authorization_policy(
        mut self,
        policy: AuthenticationTokenAuthorizationPolicy,
    ) -> Self {
        self.authorization_policy = Some(policy);
        self
    }

    pub fn build(self) -> Result<AuthTokenRequest, String> {
        let challenge = self
            .challenge
            .ok_or_else(|| "challenge is required".to_string())?;
        let ctx_type = self
            .context_type
            .ok_or_else(|| "context type is required".to_string())?;
        let ctx_value = self
            .context_value
            .ok_or_else(|| "context value is required".to_string())?;
        let subject_type = self
            .subject_identifier_type
            .ok_or_else(|| "subject identifier type is required".to_string())?;

        if let SubjectIdentifierType::CertificateFingerprint = subject_type {
            if self
                .certificate_fingerprint
                .as_ref()
                .map(|s| s.trim().is_empty())
                .unwrap_or(true)
            {
                return Err("certificate_fingerprint is required when subject type is certificateFingerprint".to_string());
            }
        }

        Ok(AuthTokenRequest {
            challenge,
            context_type: ctx_type,
            context_value: ctx_value,
            subject_identifier_type: subject_type,
            certificate_fingerprint: self.certificate_fingerprint,
            authorization_policy: self.authorization_policy,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_produces_value_and_to_xml_subject() {
        let req = AuthTokenRequestBuilder::new()
            .with_challenge("abc123")
            .with_context(ContextIdentifierType::Nip, "1234567890")
            .with_subject_type(SubjectIdentifierType::CertificateSubject)
            .build()
            .expect("build should succeed");

        let xml = req.to_xml();
        assert!(xml.contains("<?xml version=\"1.0\" encoding=\"utf-8\"?>"));
        assert!(xml.contains("xmlns=\"http://ksef.mf.gov.pl/auth/token/2.0\""));
        assert!(xml.contains("<Nip>1234567890</Nip>"));
        assert!(xml.contains("<SubjectIdentifierType>certificateSubject</SubjectIdentifierType>"));
    }

    #[test]
    fn builder_produces_value_and_to_xml_fingerprint_with_policy() {
        let mut allowed = AuthenticationTokenAllowedIps::default();
        allowed.ip4_addresses.push("192.168.0.1".to_string());
        allowed.ip4_masks.push("192.168.1.0/24".to_string());
        allowed
            .ip4_ranges
            .push("222.111.0.1-222.111.0.255".to_string());

        let policy = AuthenticationTokenAuthorizationPolicy {
            allowed_ips: allowed,
        };

        let req = AuthTokenRequestBuilder::new()
            .with_challenge("challenge-value")
            .with_context(ContextIdentifierType::Nip, "1234567890")
            .with_subject_type(SubjectIdentifierType::CertificateFingerprint)
            .with_certificate_fingerprint("70a992150f837d5b4d8c8a1c5269cef62cf500bd")
            .with_authorization_policy(policy)
            .build()
            .expect("build should succeed");

        let xml = req.to_xml();
        assert!(xml.contains("<CertificateFingerprint>70a992150f837d5b4d8c8a1c5269cef62cf500bd</CertificateFingerprint>"));
        assert!(xml.contains("<Ip4Address>192.168.0.1</Ip4Address>"));
        assert!(xml.contains("<Ip4Mask>192.168.1.0/24</Ip4Mask>"));
        assert!(xml.contains("<Ip4Range>222.111.0.1-222.111.0.255</Ip4Range>"));
        assert!(xml.contains("<Ip4Range>222.111.0.1-222.111.0.255</Ip4Range>"));
    }

    #[test]
    fn fingerprint_required_when_subject_is_fingerprint() {
        let res = AuthTokenRequestBuilder::new()
            .with_challenge("c")
            .with_context(ContextIdentifierType::Nip, "1")
            .with_subject_type(SubjectIdentifierType::CertificateFingerprint)
            .build();

        assert!(res.is_err());
    }
}
