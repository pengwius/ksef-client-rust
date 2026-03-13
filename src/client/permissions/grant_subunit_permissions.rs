use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::permissions::get_operation_status::{
    OperationStatusResponse, get_operation_status,
};
use crate::client::routes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantSubunitPermissionsRequest {
    pub subject_identifier: SubunitSubjectIdentifier,
    pub context_identifier: SubunitContextIdentifier,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subunit_name: Option<String>,
    pub subject_details: SubunitSubjectDetails,
}

impl GrantSubunitPermissionsRequest {
    pub fn builder() -> GrantSubunitPermissionsRequestBuilder {
        GrantSubunitPermissionsRequestBuilder::new()
    }
}

pub struct GrantSubunitPermissionsRequestBuilder {
    subject_identifier: Option<SubunitSubjectIdentifier>,
    context_identifier: Option<SubunitContextIdentifier>,
    description: Option<String>,
    subunit_name: Option<String>,
    subject_details: Option<SubunitSubjectDetails>,
}

impl GrantSubunitPermissionsRequestBuilder {
    pub fn new() -> Self {
        Self {
            subject_identifier: None,
            context_identifier: None,
            description: None,
            subunit_name: None,
            subject_details: None,
        }
    }

    pub fn with_subject_identifier(mut self, identifier: SubunitSubjectIdentifier) -> Self {
        self.subject_identifier = Some(identifier);
        self
    }

    pub fn with_context_identifier(mut self, identifier: SubunitContextIdentifier) -> Self {
        self.context_identifier = Some(identifier);
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_subunit_name(mut self, subunit_name: impl Into<String>) -> Self {
        self.subunit_name = Some(subunit_name.into());
        self
    }

    pub fn with_subject_details(mut self, details: SubunitSubjectDetails) -> Self {
        self.subject_details = Some(details);
        self
    }

    pub fn build(self) -> Result<GrantSubunitPermissionsRequest, String> {
        Ok(GrantSubunitPermissionsRequest {
            subject_identifier: self
                .subject_identifier
                .ok_or("subject_identifier is required")?,
            context_identifier: self
                .context_identifier
                .ok_or("context_identifier is required")?,
            description: self.description.ok_or("description is required")?,
            subunit_name: self.subunit_name,
            subject_details: self.subject_details.ok_or("subject_details is required")?,
        })
    }
}

impl Default for GrantSubunitPermissionsRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubunitSubjectIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: SubunitSubjectIdentifierType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubunitSubjectIdentifierType {
    Nip,
    Pesel,
    Fingerprint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubunitContextIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: SubunitContextIdentifierType,
    pub value: String,
}

impl SubunitContextIdentifier {
    fn normalize(&mut self) {
        if let SubunitContextIdentifierType::InternalId = self.identifier_type {
            if !self.value.contains('-') && self.value.len() > 10 {
                let (parent, rest) = self.value.split_at(10);
                self.value = format!("{}-{}", parent, rest);
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubunitContextIdentifierType {
    InternalId,
    Nip,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubunitSubjectDetails {
    pub subject_details_type: SubunitSubjectDetailsType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_by_id: Option<SubunitPersonById>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_by_fp_with_id: Option<SubunitPersonByFpWithId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_by_fp_no_id: Option<SubunitPersonByFpNoId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubunitSubjectDetailsType {
    PersonByIdentifier,
    PersonByFingerprintWithIdentifier,
    PersonByFingerprintWithoutIdentifier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubunitPersonById {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubunitPersonByFpWithId {
    pub first_name: String,
    pub last_name: String,
    pub identifier: SubunitPersonIdentifier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubunitPersonIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: SubunitPersonIdentifierType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubunitPersonIdentifierType {
    Nip,
    Pesel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubunitPersonByFpNoId {
    pub first_name: String,
    pub last_name: String,
    pub birth_date: String,
    pub id_document: SubunitIdDocument,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubunitIdDocument {
    #[serde(rename = "type")]
    pub document_type: String,
    pub number: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantSubunitPermissionsResponse {
    pub reference_number: String,
}

pub async fn grant_subunit_permissions(
    client: &KsefClient,
    request: GrantSubunitPermissionsRequest,
) -> Result<OperationStatusResponse, KsefError> {
    let url = client.url_for(routes::PERMISSIONS_SUBUNITS_GRANTS_PATH);
    let access_token = KsefClient::secret_str(&client.access_token.access_token);

    let mut req_to_send = request.clone();
    req_to_send.context_identifier.normalize();

    let resp = client
        .client
        .post(&url)
        .header("Accept", "application/json")
        .bearer_auth(access_token)
        .json(&req_to_send)
        .send()
        .await
        .map_err(KsefError::RequestError)?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(KsefError::from_api_response(status.as_u16(), body));
    }

    let mut parsed: GrantSubunitPermissionsResponse =
        resp.json().await.map_err(KsefError::RequestError)?;

    let max_attempts: usize = 10;
    let mut attempt: usize = 0;
    loop {
        let op_status = get_operation_status(client, &parsed.reference_number).await?;

        if let Some(code) = op_status.status_code() {
            if code == 200 {
                return Ok(op_status);
            }

            if code != 100 {
                if code == 410 {
                    if let SubunitContextIdentifierType::InternalId =
                        request.context_identifier.identifier_type
                    {
                        let original_ctx = &request.context_identifier.value;
                        if original_ctx.len() >= 10 {
                            let parent_nip = original_ctx.chars().take(10).collect::<String>();
                            let mut retry_req = request.clone();
                            retry_req.context_identifier = SubunitContextIdentifier {
                                identifier_type: SubunitContextIdentifierType::Nip,
                                value: parent_nip,
                            };
                            let retry_resp = client
                                .client
                                .post(&url)
                                .header("Accept", "application/json")
                                .bearer_auth(access_token)
                                .json(&retry_req)
                                .send()
                                .await
                                .map_err(KsefError::RequestError)?;

                            let retry_status = retry_resp.status();
                            if !retry_status.is_success() {
                                let body = retry_resp.text().await.unwrap_or_default();
                                return Err(KsefError::from_api_response(
                                    retry_status.as_u16(),
                                    body,
                                ));
                            }
                            let new_parsed: GrantSubunitPermissionsResponse =
                                retry_resp.json().await.map_err(KsefError::RequestError)?;
                            parsed = new_parsed;
                            attempt = 0;
                            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                            continue;
                        }
                    }
                }
                let message = op_status
                    .status_message()
                    .unwrap_or_else(|| op_status.raw.to_string());
                return Err(KsefError::ApplicationError(code as i32, message));
            }
        } else {
            return Err(KsefError::InvalidResponse(format!(
                "Unexpected operation status payload: {}",
                op_status.raw
            )));
        }

        attempt += 1;
        if attempt >= max_attempts {
            return Err(KsefError::Unexpected(format!(
                "Operation status polling timed out after {} attempts",
                max_attempts
            )));
        }

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
}
