use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizingIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizedIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryType {
    Granted,
    Received,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAuthorizationsPermissionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizing_identifier: Option<AuthorizingIdentifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_identifier: Option<AuthorizedIdentifier>,
    pub query_type: QueryType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizedEntityIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizingEntityIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubjectEntityDetails {
    pub subject_details_type: String,
    pub full_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizationGrant {
    pub id: String,
    pub author_identifier: AuthorIdentifier,
    pub authorized_entity_identifier: AuthorizedEntityIdentifier,
    pub authorizing_entity_identifier: AuthorizingEntityIdentifier,
    pub authorization_scope: String,
    pub description: String,
    pub subject_entity_details: Option<SubjectEntityDetails>,
    pub start_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAuthorizationsPermissionsResponse {
    pub authorization_grants: Vec<AuthorizationGrant>,
    pub has_more: bool,
}

pub async fn get_authorizations_permissions(
    client: &KsefClient,
    page_offset: Option<i32>,
    page_size: Option<i32>,
    request: GetAuthorizationsPermissionsRequest,
) -> Result<GetAuthorizationsPermissionsResponse, KsefError> {
    let url = client.url_for(routes::PERMISSIONS_QUERY_AUTHORIZATIONS_GRANTS_PATH);
    let access_token = &client.access_token.access_token;

    let mut req = client
        .client
        .post(&url)
        .header("Accept", "application/json")
        .bearer_auth(access_token);

    if let Some(offset) = page_offset {
        req = req.query(&[("pageOffset", offset)]);
    }
    if let Some(size) = page_size {
        req = req.query(&[("pageSize", size)]);
    }

    let resp = req
        .json(&request)
        .send()
        .await
        .map_err(KsefError::RequestError)?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(KsefError::ApiError(status.as_u16(), body));
    }

    let parsed: GetAuthorizationsPermissionsResponse =
        resp.json().await.map_err(KsefError::RequestError)?;
    Ok(parsed)
}
