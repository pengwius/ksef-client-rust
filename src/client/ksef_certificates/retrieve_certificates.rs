use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::ksef_certificates::enroll_certificate::CertificateType;
use crate::client::routes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetrieveCertificatesRequest {
    pub certificate_serial_numbers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetrieveCertificatesResponse {
    pub certificates: Vec<RetrieveCertificatesListItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetrieveCertificatesListItem {
    pub certificate: String,
    pub certificate_name: String,
    pub certificate_serial_number: String,
    pub certificate_type: CertificateType,
}

pub fn retrieve_certificates(
    client: &KsefClient,
    serial_numbers: Vec<String>,
) -> Result<Vec<RetrieveCertificatesListItem>, KsefError> {
    let fut = async {
        let url = client.url_for(routes::CERTIFICATES_RETRIEVE_PATH);
        let request = RetrieveCertificatesRequest {
            certificate_serial_numbers: serial_numbers,
        };

        let access_token = &client.access_token.access_token;

        let resp = client
            .client
            .post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .bearer_auth(access_token)
            .json(&request)
            .send()
            .await
            .map_err(KsefError::RequestError)?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(KsefError::ApiError(status.as_u16(), body));
        }

        let parsed: RetrieveCertificatesResponse =
            resp.json().await.map_err(KsefError::RequestError)?;
        Ok(parsed.certificates)
    };

    match tokio::runtime::Handle::try_current() {
        Ok(handle) => handle.block_on(fut),
        Err(_) => {
            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| KsefError::RuntimeError(e.to_string()))?;
            rt.block_on(fut)
        }
    }
}
