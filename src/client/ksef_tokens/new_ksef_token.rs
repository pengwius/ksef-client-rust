use crate::client::routes;
use crate::client::{KsefClient, KsefError};
use crate::prelude::ContextIdentifierType;
use secrecy::Secret;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct KsefToken {
    #[serde(rename = "referenceNumber")]
    pub reference_number: String,
    pub token: Secret<String>,
    #[serde(skip)]
    pub context_type: Option<ContextIdentifierType>,
    #[serde(skip)]
    pub context_value: Option<String>,
}

impl Default for KsefToken {
    fn default() -> Self {
        Self {
            reference_number: String::default(),
            token: Secret::new(String::new()),
            context_type: None,
            context_value: None,
        }
    }
}

#[derive(Default)]
pub struct KsefTokenPermissions {
    pub invoice_read: bool,
    pub invoice_write: bool,
    pub credentials_read: bool,
    pub credentials_manage: bool,
    pub subunit_manage: bool,
    pub enforcement_operations: bool,
}

pub async fn new_ksef_token(
    client: &KsefClient,
    permissions: KsefTokenPermissions,
    description: &str,
) -> Result<KsefToken, KsefError> {
    let url = client.url_for(routes::TOKENS_PATH);

    let mut permissions_vec = Vec::new();
    if permissions.invoice_read {
        permissions_vec.push("InvoiceRead");
    }
    if permissions.invoice_write {
        permissions_vec.push("InvoiceWrite");
    }
    if permissions.credentials_read {
        permissions_vec.push("CredentialsRead");
    }
    if permissions.credentials_manage {
        permissions_vec.push("CredentialsManage");
    }
    if permissions.subunit_manage {
        permissions_vec.push("SubunitManage");
    }
    if permissions.enforcement_operations {
        permissions_vec.push("EnforcementOperations");
    }

    let body = serde_json::json!({
        "permissions": permissions_vec,
        "description": if description.is_empty() { None } else { Some(description) },
    })
    .to_string();

    let token = KsefClient::secret_str(&client.access_token.access_token);
    let resp = client
        .client
        .post(&url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .body(body)
        .bearer_auth(token)
        .send()
        .await?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(KsefError::from_api_response(status.as_u16(), body));
    }

    let parsed: KsefToken = resp.json().await?;
    Ok(parsed)
}

pub async fn new_ksef_token_and_load(
    client: &mut KsefClient,
    load: bool,
    permissions: KsefTokenPermissions,
    description: &str,
) -> Result<KsefToken, KsefError> {
    let token = new_ksef_token(&*client, permissions, description).await?;
    if load {
        client.ksef_token = token.clone();
    }
    Ok(token)
}
