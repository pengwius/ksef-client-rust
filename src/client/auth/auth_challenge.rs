use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct AuthChallenge {
    pub challenge: String,
    pub timestamp: String,
    pub timestamp_ms: i64,
}

#[derive(Deserialize)]
struct AuthChallengeResponse {
    pub challenge: String,
    pub timestamp: String,
    #[serde(rename = "timestampMs")]
    pub timestamp_ms: i64,
}

pub async fn get_auth_challenge(client: &KsefClient) -> Result<AuthChallenge, KsefError> {
    let url = client.url_for(routes::AUTH_CHALLENGE_PATH);
    let http = &client.client;

    let resp = http
        .post(&url)
        .header("Accept", "application/json")
        .send()
        .await?;

    let status = resp.status();

    if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
        let retry_after_owned = resp
            .headers()
            .get("Retry-After")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_default();
        let body = resp.text().await.unwrap_or_default();
        let details = if retry_after_owned.is_empty() {
            body
        } else {
            format!("Retry-After: {}. Body: {}", retry_after_owned, body)
        };
        return Err(KsefError::ApiError(429, details));
    }

    if !status.is_success() {
        let code = status.as_u16();
        let body = resp.text().await.unwrap_or_default();
        return Err(KsefError::ApiError(code, body));
    }

    let parsed: AuthChallengeResponse = resp.json().await?;

    Ok(AuthChallenge {
        challenge: parsed.challenge,
        timestamp: parsed.timestamp,
        timestamp_ms: parsed.timestamp_ms,
    })
}

#[cfg(test)]
mod tests {
    use crate::Environment;
    use crate::{ContextIdentifier, ContextIdentifierType};

    use super::*;

    #[tokio::test]
    async fn get_auth_challenge_test() {
        let nip = "5264567890";
        let context = ContextIdentifier {
            id_type: ContextIdentifierType::Nip,
            value: nip.to_string(),
        };

        let client = KsefClient::new(Environment::Test, context);
        let result = get_auth_challenge(&client).await;
        assert!(
            result.is_ok(),
            "Expected Ok result, got Err: {:?}",
            result.err()
        );
        let challenge = result.unwrap();
        assert!(
            !challenge.challenge.is_empty(),
            "Challenge string should not be empty"
        );
        assert!(
            challenge.timestamp_ms > 0,
            "Timestamp ms should be a positive integer"
        );
    }
}
