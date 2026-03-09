use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::permissions::get_operation_status::{
    OperationStatusResponse, get_operation_status,
};
use crate::client::routes;
use serde_json::Value;

pub async fn revoke_common_permission(
    client: &KsefClient,
    permission_id: &str,
) -> Result<OperationStatusResponse, KsefError> {
    let url = client.url_for(&format!(
        "{}/{}",
        routes::PERMISSIONS_COMMON_GRANTS_PATH,
        permission_id
    ));

    let access_token = &client.access_token.access_token;

    let resp = client
        .client
        .delete(&url)
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

    let parsed_value: serde_json::Value = resp.json().await.map_err(KsefError::RequestError)?;

    let reference_opt = parsed_value
        .get("referenceNumber")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .or_else(|| {
            parsed_value
                .get("reference_number")
                .and_then(|v| v.as_str().map(|s| s.to_string()))
        });

    let handle_immediate_op =
        |op_raw: serde_json::Value| -> Result<OperationStatusResponse, KsefError> {
            let op = OperationStatusResponse::from_value(op_raw);
            if let Some(code) = op.status_code() {
                if code == 200 {
                    Ok(op)
                } else {
                    let message = op.status_message().unwrap_or_else(|| op.raw.to_string());
                    Err(KsefError::ApplicationError(code as i32, message))
                }
            } else {
                Err(KsefError::InvalidResponse(format!(
                    "Unexpected operation status payload: {}",
                    op.raw
                )))
            }
        };

    if let Some(reference_number) = reference_opt {
        let max_attempts: usize = 10;
        let mut attempt: usize = 0;
        loop {
            match get_operation_status(client, &reference_number).await {
                Ok(op_status) => {
                    if let Some(code) = op_status.status_code() {
                        if code != 100 {
                            if code == 200 {
                                return Ok(op_status);
                            } else {
                                let message = op_status
                                    .status_message()
                                    .unwrap_or_else(|| op_status.raw.to_string());
                                return Err(KsefError::ApplicationError(code as i32, message));
                            }
                        }
                    } else {
                        return Err(KsefError::InvalidResponse(format!(
                            "Unexpected operation status payload: {}",
                            op_status.raw
                        )));
                    }
                }
                Err(err) => {
                    return Err(err);
                }
            }

            attempt += 1;
            if attempt >= max_attempts {
                let final_status = get_operation_status(client, &reference_number).await?;
                if let Some(code) = final_status.status_code() {
                    if code == 200 {
                        return Ok(final_status);
                    } else {
                        let message = final_status
                            .status_message()
                            .unwrap_or_else(|| final_status.raw.to_string());
                        return Err(KsefError::ApplicationError(code as i32, message));
                    }
                } else {
                    return Err(KsefError::InvalidResponse(format!(
                        "Unexpected operation status payload on final attempt: {}",
                        final_status.raw
                    )));
                }
            }

            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    } else {
        handle_immediate_op(parsed_value)
    }
}

pub async fn get_common_permissions(client: &KsefClient) -> Result<Value, KsefError> {
    let url = client.url_for(routes::PERMISSIONS_COMMON_GRANTS_PATH);

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
        return Err(KsefError::from_api_response(status.as_u16(), body));
    }

    let parsed: Value = resp.json().await.map_err(KsefError::RequestError)?;
    Ok(parsed)
}
