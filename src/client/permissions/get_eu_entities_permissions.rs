use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEuEntitiesPermissionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_ue_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_fingerprint_identifier: Option<String>,
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
pub struct PersonIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdDocument {
    #[serde(rename = "type")]
    pub document_type: String,
    pub number: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubjectPersonDetails {
    pub subject_details_type: String,
    pub first_name: String,
    pub last_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<PersonIdentifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_document: Option<IdDocument>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubjectEntityDetails {
    pub subject_details_type: String,
    pub full_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EuEntityDetails {
    pub full_name: String,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EuEntityPermission {
    pub id: String,
    pub author_identifier: AuthorIdentifier,
    pub vat_ue_identifier: String,
    pub eu_entity_name: String,
    pub authorized_fingerprint_identifier: String,
    pub permission_scope: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_person_details: Option<SubjectPersonDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_identifier: Option<PersonIdentifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_document: Option<IdDocument>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_entity_details: Option<SubjectEntityDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_entity_details: Option<EuEntityDetails>,
    pub start_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEuEntitiesPermissionsResponse {
    pub permissions: Vec<EuEntityPermission>,
    pub has_more: bool,
}

pub async fn get_eu_entities_permissions(
    client: &KsefClient,
    page_offset: Option<i32>,
    page_size: Option<i32>,
    request_body: Option<GetEuEntitiesPermissionsRequest>,
) -> Result<GetEuEntitiesPermissionsResponse, KsefError> {
    let url = client.url_for(routes::PERMISSIONS_QUERY_EU_ENTITIES_GRANTS_PATH);

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
        let body_value = serde_json::to_value(body).unwrap_or(Value::Object(Map::new()));
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
            "DEBUG get_eu_entities_permissions -> url: {}, pageOffset: {:?}, pageSize: {:?}, body: {}",
            url, page_offset, page_size, attempted_body
        );
    }

    let resp = req.send().await.map_err(KsefError::RequestError)?;

    let status = resp.status();
    if !status.is_success() {
        let resp_body = resp.text().await.unwrap_or_default();
        eprintln!(
            "get_eu_entities_permissions ERROR -> status: {}, pageOffset: {:?}, pageSize: {:?}, request_body: {}, response_body: {}",
            status.as_u16(),
            page_offset,
            page_size,
            attempted_body,
            resp_body
        );
        return Err(KsefError::from_api_response(status.as_u16(), resp_body));
    }

    let parsed: GetEuEntitiesPermissionsResponse =
        resp.json().await.map_err(KsefError::RequestError)?;
    Ok(parsed)
}
