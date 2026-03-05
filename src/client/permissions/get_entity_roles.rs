use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentEntityIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityRole {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_entity_identifier: Option<ParentEntityIdentifier>,
    pub role: String,
    pub description: String,
    pub start_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEntityRolesResponse {
    pub roles: Vec<EntityRole>,
    pub has_more: bool,
}

pub async fn get_entity_roles(
    client: &KsefClient,
    page_offset: Option<i32>,
    page_size: Option<i32>,
) -> Result<GetEntityRolesResponse, KsefError> {
    let url = client.url_for(routes::PERMISSIONS_QUERY_ENTITIES_ROLES_PATH);

    let token = &client.access_token.access_token;
    if token.is_empty() {
        return Err(KsefError::ApplicationError(
            0,
            "No access token available".to_string(),
        ));
    }

    let mut req = client
        .client
        .get(&url)
        .header("Accept", "application/json")
        .bearer_auth(token);

    if let Some(offset) = page_offset {
        req = req.query(&[("pageOffset", offset.to_string())]);
    }
    if let Some(size) = page_size {
        req = req.query(&[("pageSize", size.to_string())]);
    }

    if cfg!(debug_assertions) {
        eprintln!(
            "DEBUG get_entity_roles -> url: {}, pageOffset: {:?}, pageSize: {:?}",
            url, page_offset, page_size
        );
    }

    let resp = req.send().await.map_err(KsefError::RequestError)?;

    let status = resp.status();
    if !status.is_success() {
        let resp_body = resp.text().await.unwrap_or_default();
        eprintln!(
            "get_entity_roles ERROR -> status: {}, pageOffset: {:?}, pageSize: {:?}, response_body: {}",
            status.as_u16(),
            page_offset,
            page_size,
            resp_body
        );
        return Err(KsefError::ApiError(status.as_u16(), resp_body));
    }

    let parsed: GetEntityRolesResponse = resp.json().await.map_err(KsefError::RequestError)?;
    Ok(parsed)
}
