use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct OpenOnlineSessionRequest {
    #[serde(rename = "formCode")]
    pub form_code: FormCode,
    #[serde(rename = "encryption")]
    pub encryption: Encryption,
}

#[derive(Debug, Serialize)]
pub struct FormCode {
    #[serde(rename = "systemCode")]
    pub system_code: String,
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct Encryption {
    #[serde(rename = "encryptedSymmetricKey")]
    pub encrypted_symmetric_key: String,
    #[serde(rename = "initializationVector")]
    pub initialization_vector: String,
}

#[derive(Debug, Deserialize)]
pub struct OpenOnlineSessionResponse {
    #[serde(rename = "referenceNumber")]
    pub reference_number: String,
    #[serde(rename = "validUntil")]
    pub valid_until: String,
}

pub struct OpenOnlineSessionRequestBuilder {
    system_code: Option<String>,
    schema_version: Option<String>,
    value: Option<String>,
    encrypted_symmetric_key: Option<Vec<u8>>,
    initialization_vector: Option<Vec<u8>>,
}

impl OpenOnlineSessionRequestBuilder {
    pub fn new() -> Self {
        Self {
            system_code: Some("FA (2)".to_string()),
            schema_version: Some("1-0E".to_string()),
            value: Some("FA".to_string()),
            encrypted_symmetric_key: None,
            initialization_vector: None,
        }
    }

    pub fn with_system_code(mut self, code: &str) -> Self {
        self.system_code = Some(code.to_string());
        self
    }

    pub fn with_schema_version(mut self, version: &str) -> Self {
        self.schema_version = Some(version.to_string());
        self
    }

    pub fn with_value(mut self, value: &str) -> Self {
        self.value = Some(value.to_string());
        self
    }

    pub fn with_encryption(mut self, key: &[u8], iv: &[u8]) -> Self {
        self.encrypted_symmetric_key = Some(key.to_vec());
        self.initialization_vector = Some(iv.to_vec());
        self
    }

    pub fn build(self) -> Result<OpenOnlineSessionRequest, KsefError> {
        let key = self
            .encrypted_symmetric_key
            .ok_or(KsefError::ApplicationError(
                0,
                "Encrypted symmetric key is required".to_string(),
            ))?;
        let iv = self
            .initialization_vector
            .ok_or(KsefError::ApplicationError(
                0,
                "Initialization vector is required".to_string(),
            ))?;

        Ok(OpenOnlineSessionRequest::new(
            self.system_code.as_deref().unwrap_or("FA"),
            self.schema_version.as_deref().unwrap_or("1-0E"),
            self.value.as_deref().unwrap_or("FA"),
            &key,
            &iv,
        ))
    }
}

impl Default for OpenOnlineSessionRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl OpenOnlineSessionRequest {
    pub fn new(
        system_code: &str,
        schema_version: &str,
        value: &str,
        encrypted_symmetric_key: &[u8],
        initialization_vector: &[u8],
    ) -> Self {
        use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

        OpenOnlineSessionRequest {
            form_code: FormCode {
                system_code: system_code.to_string(),
                schema_version: schema_version.to_string(),
                value: value.to_string(),
            },
            encryption: Encryption {
                encrypted_symmetric_key: BASE64.encode(encrypted_symmetric_key),
                initialization_vector: BASE64.encode(initialization_vector),
            },
        }
    }
}

pub fn open_online_session(
    client: &KsefClient,
    request: OpenOnlineSessionRequest,
) -> Result<OpenOnlineSessionResponse, KsefError> {
    let fut = async {
        let url = client.url_for(routes::OPEN_ONLINE_SESSION_PATH);
        let http = &client.client;

        let token = &client.access_token.access_token;
        if token.is_empty() {
            return Err(KsefError::ApplicationError(
                0,
                "No access token available".to_string(),
            ));
        }

        let resp = http
            .post(&url)
            .header("Accept", "application/json")
            .bearer_auth(token)
            .json(&request)
            .send()
            .await?;

        let status = resp.status();

        if !status.is_success() {
            let code = status.as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(KsefError::ApiError(code, body));
        }

        let parsed: OpenOnlineSessionResponse = resp.json().await?;

        Ok(parsed)
    };

    match tokio::runtime::Handle::try_current() {
        Ok(handle) => handle.block_on(fut),
        Err(_) => {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(fut)
        }
    }
}
