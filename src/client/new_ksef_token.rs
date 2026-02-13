use crate::client::routes;
use crate::client::{KsefClient, KsefError};
use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct KsefToken {
    #[serde(rename = "referenceNumber")]
    pub reference_number: String,
    pub token: String,
}

pub fn new_ksef_token(client: &KsefClient) -> Result<KsefToken, KsefError> {
    let fut = async {
        let url = client.url_for(routes::TOKENS_PATH);

        let body = serde_json::json!({
            "permissions": [
                "InvoiceRead",
                "InvoiceWrite",
                "CredentialsRead",
                "CredentialsManage",
                "SubunitManage",
                "EnforcementOperations"
            ],
            "description": "KSeF Client Rust Token",
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
