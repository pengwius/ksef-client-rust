use crate::client::auth::auth_token_request::ContextIdentifierType;
use crate::client::routes;
use crate::client::{KsefClient, KsefError};
use serde::Deserialize;

#[derive(Deserialize, Default, Debug, Clone)]
pub struct KsefToken {
    #[serde(rename = "referenceNumber")]
    pub reference_number: String,
    pub token: String,
    #[serde(skip)]
    pub context_type: Option<ContextIdentifierType>,
    #[serde(skip)]
    pub context_value: Option<String>,
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

pub fn new_ksef_token(
    client: &KsefClient,
    permissions: KsefTokenPermissions,
    description: &str,
) -> Result<KsefToken, KsefError> {
    let fut = async {
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

        let resp = client
            .client
            .post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .body(body)
            .bearer_auth(&client.access_token.access_token)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(KsefError::ApiError(status.as_u16(), body));
        }

        let parsed: KsefToken = resp.json().await?;
        Ok(parsed)
    };

    match tokio::runtime::Handle::try_current() {
        Ok(handle) => handle.block_on(fut),
        Err(_) => {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(fut)
        }
    }
}
