use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicKeyCertificate {
    pub certificate: String,
    #[serde(rename = "validFrom")]
    pub valid_from: DateTime<Utc>,
    #[serde(rename = "validTo")]
    pub valid_to: DateTime<Utc>,
    pub usage: Vec<PublicKeyCertificateUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PublicKeyCertificateUsage {
    KsefTokenEncryption,
    SymmetricKeyEncryption,
}

pub async fn get_public_key_certificates(
    client: &KsefClient,
) -> Result<Vec<PublicKeyCertificate>, KsefError> {
    let url = client.url_for(routes::PUBLIC_KEYS_PATH);
    let resp = client
        .client
        .get(&url)
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(KsefError::RequestError)?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();

        if status.as_u16() == 429 {
            return Err(KsefError::ApiError(
                429,
                format!("Too Many Requests: {}", body),
            ));
        }

        return Err(KsefError::ApiError(status.as_u16(), body));
    }

    let parsed: Vec<PublicKeyCertificate> = resp.json().await.map_err(KsefError::RequestError)?;
    Ok(parsed)
}
