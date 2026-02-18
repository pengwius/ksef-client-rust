use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevokeCertificateRequest {
    pub revocation_reason: RevocationReason,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RevocationReason {
    Unspecified,
    Superseded,
    KeyCompromise,
}

pub fn revoke_certificate(
    client: &KsefClient,
    serial_number: &str,
    reason: RevocationReason,
) -> Result<(), KsefError> {
    let fut = async {
        let url = format!(
            "{}/{}/revoke",
            client.url_for(routes::CERTIFICATES_PATH),
            serial_number
        );

        let request = RevokeCertificateRequest {
            revocation_reason: reason,
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

        Ok(())
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
