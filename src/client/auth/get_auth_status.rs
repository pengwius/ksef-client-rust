use crate::client::traits::*;
use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use serde::Deserialize;

#[derive(Deserialize)]
struct StatusResponse {
    status: StatusObject,
}

#[derive(Deserialize)]
struct StatusObject {
    code: i32,
    description: String,
}

pub async fn get_auth_status(client: &mut KsefClient) -> Result<bool, KsefError> {
    let start_time = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(120);

    loop {
        if start_time.elapsed() >= timeout {
            return Err(KsefError::TimeoutError);
        }

        let url = client.url_for(
            format!(
                "{}/{}",
                routes::AUTH_PATH,
                &client.auth_token.reference_number
            )
            .as_str(),
        );

        let resp = client
            .client
            .get(&url)
            .header("Accept", "application/json")
            .bearer_auth(KsefClient::secret_str(&client.auth_token.authentication_token))
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(KsefError::from_api_response(status.as_u16(), body));
        }

        let parsed: StatusResponse = resp.json().await?;

        match parsed.status.code {
            100 => {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                continue;
            }
            200 => match client.get_access_token().await {
                Ok(()) => return Ok(true),
                Err(e) => {
                    return Err(KsefError::Unexpected(format!(
                        "Failed to get access token: {}",
                        e
                    )));
                }
            },
            _ => {
                return Err(KsefError::ApplicationError(
                    parsed.status.code,
                    parsed.status.description,
                ));
            }
        }
    }
}
