use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct OpenBatchSessionRequest {
    #[serde(rename = "formCode")]
    pub form_code: FormCode,
    #[serde(rename = "batchFile")]
    pub batch_file: BatchFile,
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
pub struct BatchFile {
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "fileSize")]
    pub file_size: usize,
    #[serde(rename = "fileHash")]
    pub file_hash: String,
    #[serde(rename = "fileParts")]
    pub part_files: Vec<BatchFilePartInfo>,
}

#[derive(Debug, Serialize)]
pub struct BatchFilePartInfo {
    #[serde(rename = "ordinalNumber")]
    pub ordinal_number: usize,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "fileSize")]
    pub file_size: usize,
    #[serde(rename = "fileHash")]
    pub file_hash: String,
}

#[derive(Debug, Serialize)]
pub struct Encryption {
    #[serde(rename = "encryptedSymmetricKey")]
    pub encrypted_symmetric_key: String,
    #[serde(rename = "initializationVector")]
    pub initialization_vector: String,
}

#[derive(Debug, Deserialize)]
pub struct OpenBatchSessionResponse {
    #[serde(rename = "referenceNumber")]
    pub reference_number: String,
    #[serde(rename = "partUploadRequests")]
    pub part_upload_requests: Vec<PartUploadRequest>,
}

#[derive(Debug, Deserialize)]
pub struct PartUploadRequest {
    #[serde(rename = "ordinalNumber")]
    pub ordinal_number: usize,
    #[serde(rename = "method")]
    pub method: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "headers")]
    pub headers: HashMap<String, String>,
}

pub struct OpenBatchSessionRequestBuilder {
    system_code: Option<String>,
    schema_version: Option<String>,
    value: Option<String>,
    batch_file_size: Option<usize>,
    batch_file_hash: Option<String>,
    file_parts: Vec<BatchFilePartInfo>,
    encrypted_symmetric_key: Option<Vec<u8>>,
    initialization_vector: Option<Vec<u8>>,
}

impl OpenBatchSessionRequestBuilder {
    pub fn new() -> Self {
        Self {
            system_code: Some("FA (2)".to_string()),
            schema_version: Some("1-0E".to_string()),
            value: Some("FA".to_string()),
            batch_file_size: None,
            batch_file_hash: None,
            file_parts: Vec::new(),
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

    pub fn with_batch_file_info(mut self, size: usize, hash: &[u8]) -> Self {
        use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
        self.batch_file_size = Some(size);
        self.batch_file_hash = Some(BASE64.encode(hash));
        self
    }

    pub fn add_file_part(mut self, ordinal_number: usize, size: usize, hash: &[u8]) -> Self {
        use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
        self.file_parts.push(BatchFilePartInfo {
            ordinal_number,
            file_name: format!("part-{}.zip.aes", ordinal_number),
            file_size: size,
            file_hash: BASE64.encode(hash),
        });
        self
    }

    pub fn with_encryption(mut self, key: &[u8], iv: &[u8]) -> Self {
        self.encrypted_symmetric_key = Some(key.to_vec());
        self.initialization_vector = Some(iv.to_vec());
        self
    }

    pub fn build(self) -> Result<OpenBatchSessionRequest, KsefError> {
        use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

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

        let batch_file_size = self.batch_file_size.ok_or(KsefError::ApplicationError(
            0,
            "Batch file size is required".to_string(),
        ))?;
        let batch_file_hash = self.batch_file_hash.ok_or(KsefError::ApplicationError(
            0,
            "Batch file hash is required".to_string(),
        ))?;

        if self.file_parts.is_empty() {
            return Err(KsefError::ApplicationError(
                0,
                "At least one file part is required".to_string(),
            ));
        }

        Ok(OpenBatchSessionRequest {
            form_code: FormCode {
                system_code: self.system_code.unwrap_or_else(|| "FA (2)".to_string()),
                schema_version: self.schema_version.unwrap_or_else(|| "1-0E".to_string()),
                value: self.value.unwrap_or_else(|| "FA".to_string()),
            },
            batch_file: BatchFile {
                file_name: "batch.zip".to_string(),
                file_size: batch_file_size,
                file_hash: batch_file_hash,
                part_files: self.file_parts,
            },
            encryption: Encryption {
                encrypted_symmetric_key: BASE64.encode(&key),
                initialization_vector: BASE64.encode(&iv),
            },
        })
    }
}

impl Default for OpenBatchSessionRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub fn open_batch_session(
    client: &KsefClient,
    request: OpenBatchSessionRequest,
) -> Result<OpenBatchSessionResponse, KsefError> {
    let fut = async {
        let url = client.url_for(routes::OPEN_BATCH_SESSION_PATH);
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
            .bearer_auth(token);

        let body = serde_json::to_string(&request).unwrap_or_default();
        println!("OpenBatchSessionRequest body: {}", body);

        let resp = resp
            .body(body)
            .header("Content-Type", "application/json")
            .send()
            .await?;

        let status = resp.status();

        if !status.is_success() {
            let code = status.as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(KsefError::ApiError(code, body));
        }

        let parsed: OpenBatchSessionResponse = resp.json().await?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization() {
        let req = OpenBatchSessionRequest {
            form_code: FormCode {
                system_code: "FA (2)".to_string(),
                schema_version: "1-0E".to_string(),
                value: "FA".to_string(),
            },
            batch_file: BatchFile {
                file_name: "batch.zip".to_string(),
                file_size: 100,
                file_hash: "hash".to_string(),
                part_files: vec![BatchFilePartInfo {
                    ordinal_number: 1,
                    file_name: "part-1.zip.aes".to_string(),
                    file_size: 50,
                    file_hash: "part_hash".to_string(),
                }],
            },
            encryption: Encryption {
                encrypted_symmetric_key: "key".to_string(),
                initialization_vector: "iv".to_string(),
            },
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        println!("{}", json);
        assert!(json.contains("\"fileParts\":[{"));
        assert!(json.contains("\"ordinalNumber\":1"));
        assert!(json.contains("\"fileName\":\"part-1.zip.aes\""));
        assert!(json.contains("\"fileSize\":50"));
        assert!(json.contains("\"fileHash\":\"part_hash\""));
    }
}
