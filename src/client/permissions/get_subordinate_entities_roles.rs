use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubordinateEntityIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubordinateEntitiesRolesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subordinate_entity_identifier: Option<SubordinateEntityIdentifier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubordinateEntityRole {
    pub subordinate_entity_identifier: SubordinateEntityIdentifier,
    pub role: String,
    pub description: String,
    pub start_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubordinateEntitiesRolesResponse {
    pub roles: Vec<SubordinateEntityRole>,
    pub has_more: bool,
}

pub async fn get_subordinate_entities_roles(
    client: &KsefClient,
    page_offset: Option<i32>,
    page_size: Option<i32>,
    request_body: Option<GetSubordinateEntitiesRolesRequest>,
) -> Result<GetSubordinateEntitiesRolesResponse, KsefError> {
    let url = client.url_for(routes::PERMISSIONS_QUERY_SUBORDINATE_ENTITIES_ROLES_PATH);

    let token = client.access_token.access_token.expose_secret();
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

        if let Some(sub_id) = body_value.get_mut("subordinateEntityIdentifier") {
            if let Some(id_type) = sub_id.get("type") {
                if id_type == "InternalId" {
                    if let Some(v) = sub_id.get_mut("value") {
                        if let Some(s) = v.as_str() {
                            if !s.contains('-') {
                                let digits_only: String =
                                    s.chars().filter(|c| c.is_ascii_digit()).collect();
                                if digits_only.len() > 10 {
                                    let (nip, rest) = digits_only.split_at(10);
                                    let hyphenated = format!("{}-{}", nip, rest);
                                    *v = Value::String(hyphenated);
                                }
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
            "DEBUG get_subordinate_entities_roles -> url: {}, pageOffset: {:?}, pageSize: {:?}, body: {}",
            url, page_offset, page_size, attempted_body
        );
    }

    let resp = req.send().await.map_err(KsefError::RequestError)?;

    let status = resp.status();
    if !status.is_success() {
        let resp_body = resp.text().await.unwrap_or_default();
        eprintln!(
            "get_subordinate_entities_roles ERROR -> status: {}, pageOffset: {:?}, pageSize: {:?}, request_body: {}, response_body: {}",
            status.as_u16(),
            page_offset,
            page_size,
            attempted_body,
            resp_body
        );
        return Err(KsefError::from_api_response(status.as_u16(), resp_body));
    }

    let parsed: GetSubordinateEntitiesRolesResponse =
        resp.json().await.map_err(KsefError::RequestError)?;
    Ok(parsed)
}
