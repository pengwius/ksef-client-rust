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

pub fn get_auth_status(client: &KsefClient) -> Result<bool, KsefError> {
    let start_time = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(120);

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| KsefError::RuntimeError(e.to_string()))?;

    loop {
        if start_time.elapsed() >= timeout {
            return Err(KsefError::TimeoutError);
        }

        let fut = async {
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
                .bearer_auth(&client.auth_token.authentication_token)
                .send()
                .await?;

            let status = resp.status();
            if !status.is_success() {
                let body = resp.text().await.unwrap_or_default();
                return Err(KsefError::ApiError(status.as_u16(), body));
            }

            let parsed: StatusResponse = resp.json().await?;
            Ok(parsed)
        };

        let status_response = rt.block_on(fut)?;

        match status_response.status.code {
            100 => {
                std::thread::sleep(std::time::Duration::from_secs(1));
                continue;
            }
            200 => {
                return Ok(true);
            }
            _ => {
                return Err(KsefError::ApplicationError(
                    status_response.status.code,
                    status_response.status.description,
                ));
            }
        }
    }
}
