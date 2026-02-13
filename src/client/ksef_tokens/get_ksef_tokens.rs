use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryTokensResponse {
    pub continuation_token: Option<String>,
    pub tokens: Vec<DetailedKsefToken>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailedKsefToken {
    pub reference_number: String,
    pub author_identifier: TokenIdentifier,
    pub context_identifier: ContextIdentifier,
    pub description: String,
    pub requested_permissions: Vec<TokenPermissionType>,
    pub date_created: DateTime<Utc>,
    pub last_use_date: Option<DateTime<Utc>>,
    pub status: TokenStatus,
    pub status_details: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenIdentifier {
    #[serde(rename = "type")]
    pub id_type: IdentifierType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IdentifierType {
    Nip,
    Pesel,
    Fingerprint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextIdentifier {
    #[serde(rename = "type")]
    pub id_type: ContextIdentifierType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ContextIdentifierType {
    Nip,
    InternalId,
    NipVatUe,
    PeppolId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TokenPermissionType {
    InvoiceRead,
    InvoiceWrite,
    CredentialsRead,
    CredentialsManage,
    SubunitManage,
    EnforcementOperations,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TokenStatus {
    Pending,
    Active,
    Revoking,
    Revoked,
    Failed,
}

pub fn get_ksef_tokens(client: &KsefClient) -> Result<Vec<DetailedKsefToken>, KsefError> {
    let fut = async {
        let url = client.url_for(routes::TOKENS_PATH);

        let resp = client
            .client
            .get(&url)
            .header("Accept", "application/json")
            .bearer_auth(&client.access_token.access_token)
            .send()
            .await
            .map_err(KsefError::RequestError)?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(KsefError::ApiError(status.as_u16(), body));
        }

        let parsed: QueryTokensResponse = resp.json().await.map_err(KsefError::RequestError)?;
        Ok(parsed.tokens)
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
