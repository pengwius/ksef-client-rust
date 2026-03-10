use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateLimits {
    pub can_request: bool,
    pub enrollment: LimitDetails,
    pub certificate: LimitDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitDetails {
    pub remaining: i32,
    pub limit: i32,
}

pub async fn get_certificates_limits(client: &KsefClient) -> Result<CertificateLimits, KsefError> {
    let url = client.url_for(routes::CERTIFICATES_LIMITS_PATH);

    let access_token = KsefClient::secret_str(&client.access_token.access_token);

    let resp = client
        .client
        .get(&url)
        .header("Accept", "application/json")
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(KsefError::RequestError)?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(KsefError::from_api_response(status.as_u16(), body));
    }

    let parsed: CertificateLimits = resp.json().await.map_err(KsefError::RequestError)?;
    Ok(parsed)
}
