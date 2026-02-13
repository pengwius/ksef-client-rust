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
