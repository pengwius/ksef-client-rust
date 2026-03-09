use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntitiesContextIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: String,
    pub value: String,
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

    let token = &client.access_token.access_token;
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

    if let Some(ref body) = request_body {
        let mut body_value = serde_json::to_value(body).unwrap_or(Value::Object(Map::new()));

        if let Some(ctx_id_val) = body_value.get_mut("contextIdentifier") {
            if let Some(id_type_val) = ctx_id_val.get("type") {
                if id_type_val == "InternalId" {
                    if let Some(v) = ctx_id_val.get_mut("value") {
                        if let Some(s) = v.as_str() {
                            if !s.contains('-') {
                                let digits_only: String =
                                    s.chars().filter(|c| c.is_ascii_digit()).collect();
                                if digits_only.len() > 10 {
                                    let (nip, rest) = digits_only.split_at(10);
                                    let hyphenated = format!("{}-{}", nip, rest);
                                    *v = Value::String(hyphenated);
                                }
                            } else {
                            }
                        }
                    }
                }
            }
        }

        req = req.json(&body_value);
    } else {
        req = req.json(&Value::Object(Map::new()));
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
