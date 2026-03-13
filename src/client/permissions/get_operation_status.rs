use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationStatusResponse {
    pub raw: Value,
    #[serde(skip)]
    pub code: Option<i32>,
    #[serde(skip)]
    pub message: Option<String>,
    #[serde(skip)]
    pub details: Option<Vec<String>>,
}

impl OperationStatusResponse {
    pub fn status_code(&self) -> Option<i32> {
        self.code
    }

    pub fn status_message(&self) -> Option<String> {
        self.message.clone()
    }

    pub fn status_details(&self) -> Option<Vec<String>> {
        self.details.clone()
    }

    pub fn from_value(raw: Value) -> Self {
        let (code, message, details) = parse_operation_fields(&raw);
        OperationStatusResponse {
            raw,
            code,
            message,
            details,
        }
    }
}

fn parse_operation_fields(v: &Value) -> (Option<i32>, Option<String>, Option<Vec<String>>) {
    fn value_to_i32(v: &Value) -> Option<i32> {
        if let Some(n) = v.as_i64() {
            return Some(n as i32);
        }
        if let Some(s) = v.as_str() {
            if let Ok(n) = s.parse::<i32>() {
                return Some(n);
            }
        }
        None
    }

    let mut code: Option<i32> = None;
    let mut message: Option<String> = None;
    let mut details: Option<Vec<String>> = None;

    match v {
        Value::Number(_) | Value::String(_) => {
            code = value_to_i32(v);
        }
        Value::Object(map) => {
            if let Some(val) = map.get("code").or_else(|| map.get("status")) {
                if let Some(n) = value_to_i32(val) {
                    code = Some(n);
                } else if let Value::Object(nested) = val {
                    if let Some(v2) = nested.get("code").or_else(|| nested.get("status")) {
                        if let Some(n) = value_to_i32(v2) {
                            code = Some(n);
                        }
                    }
                }
            }
            if code.is_none() {
                if let Some(Value::Object(status_obj)) = map.get("status") {
                    if let Some(v2) = status_obj.get("code").or_else(|| status_obj.get("status")) {
                        if let Some(n) = value_to_i32(v2) {
                            code = Some(n);
                        }
                    }
                }
            }

            if let Some(Value::String(s)) = map.get("description").or_else(|| map.get("message")) {
                message = Some(s.clone());
            } else if let Some(Value::Array(arr)) = map.get("details") {
                let mut collected = Vec::new();
                for item in arr.iter() {
                    if let Some(s) = item.as_str() {
                        collected.push(s.to_string());
                    } else if let Some(obj) = item.as_object() {
                        if let Some(Value::String(s)) =
                            obj.get("description").or_else(|| obj.get("message"))
                        {
                            collected.push(s.clone());
                        }
                    }
                }
                if !collected.is_empty() {
                    message = message.or_else(|| collected.get(0).cloned());
                    details = Some(collected);
                }
            } else if let Some(Value::String(s)) = map.get("message") {
                message = Some(s.clone());
            }
        }
        _ => {}
    }

    (code, message, details)
}

pub async fn get_operation_status(
    client: &KsefClient,
    reference_number: &str,
) -> Result<OperationStatusResponse, KsefError> {
    let url = client.url_for(&format!(
        "{}/{}",
        routes::PERMISSIONS_OPERATIONS_PATH,
        reference_number
    ));

    let token = client.access_token.access_token.expose_secret();
    if token.is_empty() {
        return Err(KsefError::ApplicationError(
            0,
            "No access token available".to_string(),
        ));
    }

    let resp = client
        .client
        .get(&url)
        .header("Accept", "application/json")
        .bearer_auth(token)
        .send()
        .await
        .map_err(KsefError::RequestError)?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(KsefError::from_api_response(status.as_u16(), body));
    }

    let parsed_value: Value = resp.json().await.map_err(KsefError::RequestError)?;

    let (code, message, details) = parse_operation_fields(&parsed_value);

    Ok(OperationStatusResponse {
        raw: parsed_value,
        code,
        message,
        details,
    })
}

pub async fn poll_operation_status(
    client: &KsefClient,
    reference_number: &str,
    max_attempts: usize,
    delay_ms: u64,
) -> Result<OperationStatusResponse, KsefError> {
    let mut attempt = 0;
    loop {
        let status = get_operation_status(client, reference_number).await?;

        if let Some(code) = status.status_code() {
            if code != 100 {
                if code == 200 {
                    return Ok(status);
                } else {
                    let msg = status
                        .status_message()
                        .unwrap_or_else(|| status.raw.to_string());
                    return Err(KsefError::ApplicationError(code, msg));
                }
            }
        } else {
            return Err(KsefError::InvalidResponse(format!(
                "Unexpected operation status payload: {}",
                status.raw
            )));
        }

        attempt += 1;
        if attempt >= max_attempts {
            return Err(KsefError::Unexpected(format!(
                "Operation status polling timed out after {} attempts",
                max_attempts
            )));
        }

        tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
    }
}

pub async fn process_status_response(
    client: &KsefClient,
    response_body: Value,
    max_attempts: usize,
    delay_ms: u64,
) -> Result<OperationStatusResponse, KsefError> {
    let reference_opt = response_body
        .get("referenceNumber")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .or_else(|| {
            response_body
                .get("reference_number")
                .and_then(|v| v.as_str().map(|s| s.to_string()))
        });

    if let Some(reference_number) = reference_opt {
        poll_operation_status(client, &reference_number, max_attempts, delay_ms).await
    } else {
        let op = OperationStatusResponse::from_value(response_body);
        if let Some(code) = op.status_code() {
            if code == 200 {
                Ok(op)
            } else {
                let message = op.status_message().unwrap_or_else(|| op.raw.to_string());
                Err(KsefError::ApplicationError(code, message))
            }
        } else {
            Err(KsefError::InvalidResponse(format!(
                "Unexpected operation status payload: {}",
                op.raw
            )))
        }
    }
}
