use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnrollmentData {
    pub common_name: String,
    pub country_name: String,
    pub given_name: Option<String>,
    pub surname: Option<String>,
    pub serial_number: Option<String>,
    pub unique_identifier: Option<String>,
    pub organization_name: Option<String>,
    pub organization_identifier: Option<String>,
}

pub fn get_enrollment_data(client: &KsefClient) -> Result<EnrollmentData, KsefError> {
    let fut = async {
        let url = client.url_for(routes::CERTIFICATES_ENROLLMENT_DATA_PATH);

        let access_token = &client.access_token.access_token;

        let resp = client
            .client
            .get(&url)
            .header("Accept", "application/json")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(KsefError::RequestError)?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(KsefError::ApiError(status.as_u16(), body));
        }

        let parsed: EnrollmentData = resp.json().await.map_err(KsefError::RequestError)?;
        Ok(parsed)
    };

    match tokio::runtime::Handle::try_current() {
        Ok(handle) => handle.block_on(fut),
        Err(_) => {
            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| KsefError::RuntimeError(e.to_string()))?;
            rt.block_on(fut)
        }
    }
}
