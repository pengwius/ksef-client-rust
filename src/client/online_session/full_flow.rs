use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::online_session::close_online_session::close_online_session;
use crate::client::online_session::encryption::generate_encryption_data;
use crate::client::online_session::open_online_session::{
    OpenOnlineSessionRequestBuilder, open_online_session,
};
use crate::client::online_session::send_invoice::send_invoice;
use crate::client::types::ReferenceNumber;

#[derive(Debug, Clone)]
pub struct OnlineSubmissionResult {
    pub session_reference_number: ReferenceNumber,
    pub invoice_reference_number: ReferenceNumber,
}

pub async fn submit_online(
    client: &KsefClient,
    invoice: &[u8],
    system_code: Option<&str>,
    schema_version: Option<&str>,
    value: Option<&str>,
) -> Result<OnlineSubmissionResult, KsefError> {
    let encryption_data = generate_encryption_data(client).await?;

    let mut builder = OpenOnlineSessionRequestBuilder::new()
        .with_encryption(
            &encryption_data.encrypted_symmetric_key,
            &encryption_data.initialization_vector,
        );

    if let Some(code) = system_code {
        builder = builder.with_system_code(code);
    }
    if let Some(version) = schema_version {
        builder = builder.with_schema_version(version);
    }
    if let Some(v) = value {
        builder = builder.with_value(v);
    }

    let request = builder.build()?;

    let session_response = open_online_session(client, request).await?;
    let session_reference_number = ReferenceNumber::new(session_response.reference_number);

    let send_result =
        send_invoice(client, &session_reference_number, invoice, &encryption_data).await?;
    let invoice_reference_number = ReferenceNumber::new(send_result.reference_number);

    close_online_session(client, &session_reference_number).await?;

    Ok(OnlineSubmissionResult {
        session_reference_number,
        invoice_reference_number,
    })
}
