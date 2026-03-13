use crate::client::KsefClient;
use crate::client::batch_session::close_batch_session::close_batch_session;
use crate::client::batch_session::open_batch_session::{
    OpenBatchSessionRequestBuilder, open_batch_session,
};
use crate::client::batch_session::upload_batch_parts::upload_batch_parts;
use crate::client::batch_session::zip::{create_zip, encrypt_zip_parts, split_zip};
use crate::client::error::KsefError;
use crate::client::online_session::encryption::generate_encryption_data;
use crate::invoices::InvoicePayload;

#[derive(Debug, Clone)]
pub struct BatchSubmissionResult {
    pub reference_number: String,
    pub number_of_parts: usize,
    pub total_size_bytes: usize,
}

pub async fn submit_batch(
    client: &KsefClient,
    invoices: &[InvoicePayload],
    max_part_size_bytes: Option<usize>,
) -> Result<BatchSubmissionResult, KsefError> {
    let zip_result = create_zip(invoices)?;
    let total_size = zip_result.metadata.size;

    let max_size = max_part_size_bytes.unwrap_or(50 * 1024 * 1024);
    let raw_parts = split_zip(&zip_result.content, max_size);

    let encryption_data = generate_encryption_data(client).await?;

    let encrypted_parts = encrypt_zip_parts(
        &raw_parts,
        &encryption_data.symmetric_key,
        &encryption_data.initialization_vector,
    )?;

    let mut builder = OpenBatchSessionRequestBuilder::new()
        .with_batch_file_info(zip_result.metadata.size, &zip_result.metadata.hash)
        .with_encryption(
            &encryption_data.encrypted_symmetric_key,
            &encryption_data.initialization_vector,
        );

    for part in encrypted_parts.iter() {
        builder =
            builder.add_file_part(part.ordinal_number, part.metadata.size, &part.metadata.hash);
    }

    let open_request = builder.build()?;

    let session_response = open_batch_session(client, open_request).await?;
    let reference_number = session_response.reference_number.clone();

    upload_batch_parts(client, &session_response, &encrypted_parts).await?;

    close_batch_session(client, &reference_number).await?;

    Ok(BatchSubmissionResult {
        reference_number,
        number_of_parts: encrypted_parts.len(),
        total_size_bytes: total_size,
    })
}
