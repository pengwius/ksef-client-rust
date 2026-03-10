use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    #[serde(rename = "type")]
    pub identifier_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl Identifier {
    pub fn builder() -> IdentifierBuilder {
        IdentifierBuilder::new()
    }
}

pub struct IdentifierBuilder {
    identifier_type: Option<String>,
    value: Option<String>,
}

impl IdentifierBuilder {
    pub fn new() -> Self {
        Self {
            identifier_type: None,
            value: None,
        }
    }

    pub fn with_type(mut self, t: impl Into<String>) -> Self {
        self.identifier_type = Some(t.into());
        self
    }

    pub fn with_value(mut self, v: impl Into<String>) -> Self {
        self.value = Some(v.into());
        self
    }

    pub fn build(self) -> Result<Identifier, String> {
        Ok(Identifier {
            identifier_type: self.identifier_type.ok_or("identifier_type is required")?,
            value: self.value,
        })
    }
}

impl Default for IdentifierBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonsPermissionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_identifier: Option<Identifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_identifier: Option<Identifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_identifier: Option<Identifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_identifier: Option<Identifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_state: Option<String>,
    pub query_type: String,
}

impl PersonsPermissionsRequest {
    pub fn builder() -> PersonsPermissionsRequestBuilder {
        PersonsPermissionsRequestBuilder::new()
    }
}

pub struct PersonsPermissionsRequestBuilder {
    author_identifier: Option<Identifier>,
    authorized_identifier: Option<Identifier>,
    context_identifier: Option<Identifier>,
    target_identifier: Option<Identifier>,
    permission_types: Option<Vec<String>>,
    permission_state: Option<String>,
    query_type: Option<String>,
}

impl PersonsPermissionsRequestBuilder {
    pub fn new() -> Self {
        Self {
            author_identifier: None,
            authorized_identifier: None,
            context_identifier: None,
            target_identifier: None,
            permission_types: None,
            permission_state: None,
            query_type: None,
        }
    }

    pub fn with_author_identifier(mut self, id: Identifier) -> Self {
        self.author_identifier = Some(id);
        self
    }

    pub fn with_authorized_identifier(mut self, id: Identifier) -> Self {
        self.authorized_identifier = Some(id);
        self
    }

    pub fn with_context_identifier(mut self, id: Identifier) -> Self {
        self.context_identifier = Some(id);
        self
    }

    pub fn with_target_identifier(mut self, id: Identifier) -> Self {
        self.target_identifier = Some(id);
        self
    }

    pub fn with_permission_types(mut self, types: Vec<String>) -> Self {
        self.permission_types = Some(types);
        self
    }

    pub fn with_permission_state(mut self, state: impl Into<String>) -> Self {
        self.permission_state = Some(state.into());
        self
    }

    pub fn with_query_type(mut self, qt: impl Into<String>) -> Self {
        self.query_type = Some(qt.into());
        self
    }

    pub fn build(self) -> Result<PersonsPermissionsRequest, String> {
        Ok(PersonsPermissionsRequest {
            author_identifier: self.author_identifier,
            authorized_identifier: self.authorized_identifier,
            context_identifier: self.context_identifier,
            target_identifier: self.target_identifier,
            permission_types: self.permission_types,
            permission_state: self.permission_state,
            query_type: self.query_type.ok_or("query_type is required")?,
        })
    }
}

impl Default for PersonsPermissionsRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
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
pub struct PersonPermission {
    pub id: String,
    pub authorized_identifier: PersonIdentifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_identifier: Option<Identifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_identifier: Option<Identifier>,
    pub author_identifier: PersonIdentifier,
    pub permission_scope: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_person_details: Option<SubjectPersonDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_entity_details: Option<SubjectEntityDetails>,
    pub permission_state: String,
    pub start_date: String,
    pub can_delegate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPersonsPermissionsResponse {
    pub permissions: Vec<PersonPermission>,
    pub has_more: bool,
}

pub async fn get_persons_permissions(
    client: &KsefClient,
    page_offset: Option<i32>,
    page_size: Option<i32>,
    request_body: Option<PersonsPermissionsRequest>,
) -> Result<GetPersonsPermissionsResponse, KsefError> {
    let url = client.url_for(routes::PERMISSIONS_QUERY_PERSONS_GRANTS_PATH);

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

    let parsed: GetPersonsPermissionsResponse =
        resp.json().await.map_err(KsefError::RequestError)?;
    Ok(parsed)
}
