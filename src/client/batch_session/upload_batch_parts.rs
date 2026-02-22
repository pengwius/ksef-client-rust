use crate::client::KsefClient;
use crate::client::batch_session::open_batch_session::OpenBatchSessionResponse;
use crate::client::batch_session::zip::EncryptedBatchPart;
use crate::client::error::KsefError;
use reqwest::Method;
use std::str::FromStr;

pub fn upload_batch_parts(
    client: &KsefClient,
    session_response: &OpenBatchSessionResponse,
    parts: &[EncryptedBatchPart],
) -> Result<(), KsefError> {
    let fut = async {
        for request in &session_response.part_upload_requests {
            let part = parts
                .iter()
                .find(|p| p.ordinal_number == request.ordinal_number)
                .ok_or_else(|| {
                    KsefError::ApplicationError(
                        0,
                        format!(
                            "No encrypted part found for ordinal number {}",
                            request.ordinal_number
                        ),
                    )
                })?;

            let method = Method::from_str(&request.method).map_err(|e| {
                KsefError::ApplicationError(0, format!("Invalid HTTP method: {}", e))
            })?;

            let mut req_builder = client
                .client
                .request(method, &request.url)
                .body(part.content.clone());

            for (key, value) in &request.headers {
                req_builder = req_builder.header(key, value);
            }

            let resp = req_builder.send().await?;
            let status = resp.status();

            if !status.is_success() {
                let code = status.as_u16();
                let body = resp.text().await.unwrap_or_default();
                return Err(KsefError::ApiError(code, body));
            }
        }

        Ok(())
    };

    match tokio::runtime::Handle::try_current() {
        Ok(handle) => handle.block_on(fut),
        Err(_) => {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(fut)
        }
    }
}
