use crate::client::KsefClient;
use crate::client::error::KsefError;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::client::routes;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    #[serde(rename = "type")]
    pub identifier_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPersonalPermissionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_identifier: Option<Identifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_identifier: Option<Identifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizedIdentifier {
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
    pub person_identifier: Option<PersonIdentifier>,
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
pub struct PersonalPermission {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_identifier: Option<Identifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_identifier: Option<AuthorizedIdentifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_identifier: Option<Identifier>,
    pub permission_scope: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_person_details: Option<SubjectPersonDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_entity_details: Option<SubjectEntityDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_identifier: Option<PersonIdentifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_document: Option<IdDocument>,
    pub permission_state: String,
    pub start_date: String,
    pub can_delegate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPersonalPermissionsResponse {
    pub permissions: Vec<PersonalPermission>,
    pub has_more: bool,
}

pub async fn get_personal_permissions(
    client: &KsefClient,
    page_offset: Option<i32>,
    page_size: Option<i32>,
    request_body: Option<GetPersonalPermissionsRequest>,
) -> Result<GetPersonalPermissionsResponse, KsefError> {
    let url = client.url_for(routes::PERMISSIONS_QUERY_PERSONAL_GRANTS_PATH);

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
        .bearer_auth(token);

    if let Some(offset) = page_offset {
        req = req.query(&[("pageOffset", offset)]);
    }
    if let Some(size) = page_size {
        req = req.query(&[("pageSize", size)]);
    }

    if let Some(body) = request_body {
        req = req.json(&body);
    } else {
        req = req.json(&Value::Object(serde_json::Map::new()));
    }

    let resp = req.send().await.map_err(KsefError::RequestError)?;
    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(KsefError::from_api_response(status.as_u16(), body));
    }

    let parsed: GetPersonalPermissionsResponse =
        resp.json().await.map_err(KsefError::RequestError)?;
    Ok(parsed)
}
