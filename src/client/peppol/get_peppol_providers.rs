use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeppolProvider {
    pub id: String,

    pub name: String,

    pub date_created: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPeppolProvidersResponse {
    #[serde(rename = "peppolProviders")]
    pub peppol_providers: Vec<PeppolProvider>,

    pub has_more: bool,
}

pub async fn get_peppol_providers(
    client: &KsefClient,
    page_size: Option<i32>,
    page_offset: Option<i32>,
) -> Result<GetPeppolProvidersResponse, KsefError> {
    let url = client.url_for(routes::PEPPOL_QUERY_PATH);

    let mut query_params = Vec::new();
    if let Some(size) = page_size {
        query_params.push(("pageSize", size.to_string()));
    }
    if let Some(offset) = page_offset {
        query_params.push(("pageOffset", offset.to_string()));
    }

    let access_token = &client.access_token.access_token;

    let resp = client
        .client
        .get(&url)
        .query(&query_params)
        .header("Accept", "application/json")
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(KsefError::RequestError)?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(KsefError::from_api_response(status.as_u16(), body));
    }

    let parsed: GetPeppolProvidersResponse = resp.json().await.map_err(KsefError::RequestError)?;
    Ok(parsed)
}
