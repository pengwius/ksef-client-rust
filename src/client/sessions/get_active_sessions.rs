use crate::client::KsefClient;
use crate::client::KsefError;
use crate::client::routes;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub start_date: DateTime<Utc>,
    pub authentication_method: AuthenticationMethod,
    pub status: SessionStatus,
    pub is_token_redeemed: Option<bool>,
    pub last_token_refresh_date: Option<DateTime<Utc>>,
    pub refresh_token_valid_until: Option<DateTime<Utc>>,
    pub reference_number: String,
    pub is_current: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionStatus {
    pub code: i32,
    pub description: String,
    pub details: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum AuthenticationMethod {
    Token,
    TrustedProfile,
    InternalCertificate,
    QualifiedSignature,
    QualifiedSeal,
    PersonalSignature,
    PeppolSignature,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuerySessionsResponse {
    pub continuation_token: Option<String>,
    pub items: Vec<Session>,
}

pub fn get_active_sessions(
    client: &KsefClient,
    continuation_token: Option<&str>,
) -> Result<QuerySessionsResponse, KsefError> {
    let fut = async {
        let url = client.url_for(routes::AUTH_SESSIONS_PATH);

        let access_token = &client.access_token.access_token;

        let mut req = client
            .client
            .get(&url)
            .header("Accept", "application/json")
            .bearer_auth(access_token);

        if let Some(token) = continuation_token {
            req = req.header("x-continuation-token", token);
        }

        let resp = req.send().await.map_err(KsefError::RequestError)?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(KsefError::ApiError(status.as_u16(), body));
        }

        let parsed: QuerySessionsResponse = resp.json().await.map_err(KsefError::RequestError)?;
        Ok(parsed)
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
