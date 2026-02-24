use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::online_session::close_online_session::close_online_session;
use crate::client::online_session::encryption::generate_encryption_data;
use crate::client::online_session::open_online_session::{
    OpenOnlineSessionRequestBuilder, open_online_session,
};
use crate::client::online_session::send_invoice::send_invoice;

#[derive(Debug, Clone)]
pub struct OnlineSubmissionResult {
    pub session_reference_number: String,
    pub invoice_reference_number: String,
}

pub fn submit_online(
    client: &KsefClient,
    invoice: &[u8],
) -> Result<OnlineSubmissionResult, KsefError> {
    let encryption_data = generate_encryption_data(client)?;

    let request = OpenOnlineSessionRequestBuilder::new()
        .with_encryption(
            &encryption_data.encrypted_symmetric_key,
            &encryption_data.initialization_vector,
        )
        .build()?;

    let session_response = open_online_session(client, request)?;
    let session_reference_number = session_response.reference_number.clone();

    let send_result = send_invoice(client, &session_reference_number, invoice, &encryption_data)?;
    let invoice_reference_number = send_result.reference_number;

    close_online_session(client, &session_reference_number)?;

    Ok(OnlineSubmissionResult {
        session_reference_number,
        invoice_reference_number,
    })
}
