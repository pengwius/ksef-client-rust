use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntitiesContextIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: String,
    pub value: String,
}

impl EntitiesContextIdentifier {
    fn normalize(&mut self) {
        if self.identifier_type == "InternalId" && !self.value.contains('-') {
            let digits_only: String = self.value.chars().filter(|c| c.is_ascii_digit()).collect();
            if digits_only.len() > 10 {
                let (nip, rest) = digits_only.split_at(10);
                self.value = format!("{}-{}", nip, rest);
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEntitiesPermissionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_identifier: Option<EntitiesContextIdentifier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityPermissionItem {
    pub id: String,
    pub context_identifier: EntitiesContextIdentifier,
    pub permission_scope: String,
    pub description: String,
    pub start_date: String,
    pub can_delegate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEntitiesPermissionsResponse {
    pub permissions: Vec<EntityPermissionItem>,
    pub has_more: bool,
}

pub async fn get_entities_permissions(
    client: &KsefClient,
    page_offset: Option<i32>,
    page_size: Option<i32>,
    request_body: Option<GetEntitiesPermissionsRequest>,
) -> Result<GetEntitiesPermissionsResponse, KsefError> {
    let url = client.url_for(routes::PERMISSIONS_QUERY_ENTITIES_GRANTS_PATH);

    let token = KsefClient::secret_str(&client.access_token.access_token);
    if token.is_empty() {
        return Err(KsefError::ApplicationError(
            0,
            "No access token available".to_string(),
        ));
    }

    let mut req = client
        .client
        .post(&url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .bearer_auth(token);

    if let Some(offset) = page_offset {
        req = req.query(&[("pageOffset", offset.to_string())]);
    }
    if let Some(size) = page_size {
        req = req.query(&[("pageSize", size.to_string())]);
    }

    if let Some(body) = &request_body {
        let mut body_clone = body.clone();
        if let Some(ctx_id) = &mut body_clone.context_identifier {
            ctx_id.normalize();
        }
        req = req.json(&body_clone);
    } else {
        req = req.json(&serde_json::json!({}));
    }

    let attempted_body = match &request_body {
        Some(b) => {
            serde_json::to_string(b).unwrap_or_else(|_| "<failed to serialize body>".to_string())
        }
        None => "{}".to_string(),
    };

    if cfg!(debug_assertions) {
        eprintln!(
            "DEBUG get_entities_permissions -> url: {}, pageOffset: {:?}, pageSize: {:?}, body: {}",
            url, page_offset, page_size, attempted_body
        );
    }

    let resp = req.send().await.map_err(KsefError::RequestError)?;

    let status = resp.status();
    if !status.is_success() {
        let resp_body = resp.text().await.unwrap_or_default();
        eprintln!(
            "get_entities_permissions ERROR -> status: {}, pageOffset: {:?}, pageSize: {:?}, request_body: {}, response_body: {}",
            status.as_u16(),
            page_offset,
            page_size,
            attempted_body,
            resp_body
        );
        return Err(KsefError::from_api_response(status.as_u16(), resp_body));
    }

    let parsed: GetEntitiesPermissionsResponse =
        resp.json().await.map_err(KsefError::RequestError)?;
    Ok(parsed)
}
