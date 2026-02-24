use crate::client::error::KsefError;
use crate::client::online_session::encryption::encrypt_invoice;
use openssl::sha::sha256;
use std::io::{Cursor, Write};
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

#[derive(Debug, Clone)]
pub struct InvoicePayload {
    pub filename: String,
    pub content: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub hash: Vec<u8>,
    pub size: usize,
}

pub struct BatchZipResult {
    pub content: Vec<u8>,
    pub metadata: FileMetadata,
}

pub struct EncryptedBatchPart {
    pub content: Vec<u8>,
    pub metadata: FileMetadata,
    pub ordinal_number: usize,
}

pub fn encrypt_zip_parts(
    parts: &[Vec<u8>],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<EncryptedBatchPart>, KsefError> {
    let mut encrypted_parts = Vec::with_capacity(parts.len());

    for (i, part) in parts.iter().enumerate() {
        let encrypted_content = encrypt_invoice(part, key, iv)?;
        let hash = sha256(&encrypted_content);
        let size = encrypted_content.len();

        encrypted_parts.push(EncryptedBatchPart {
            content: encrypted_content,
            metadata: FileMetadata {
                hash: hash.to_vec(),
                size,
            },
            ordinal_number: i + 1,
        });
    }

    Ok(encrypted_parts)
}

pub fn split_zip(content: &[u8], max_part_size: usize) -> Vec<Vec<u8>> {
    if content.is_empty() {
        return Vec::new();
    }

    let len = content.len();
    let part_count = (len as f64 / max_part_size as f64).ceil() as usize;

    if part_count == 0 {
        return Vec::new();
    }

    let part_size = (len as f64 / part_count as f64).ceil() as usize;

    let mut parts = Vec::with_capacity(part_count);

    for i in 0..part_count {
        let start = i * part_size;
        if start >= len {
            break;
        }
        let end = std::cmp::min(start + part_size, len);

        parts.push(content[start..end].to_vec());
    }

    parts
}

pub fn create_zip(invoices: &[InvoicePayload]) -> Result<BatchZipResult, KsefError> {
    let buffer = Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(buffer);

    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    for invoice in invoices {
        zip.start_file(&invoice.filename, options).map_err(|e| {
            KsefError::ApplicationError(0, format!("Failed to start file in zip: {}", e))
        })?;
        zip.write_all(&invoice.content).map_err(|e| {
            KsefError::ApplicationError(0, format!("Failed to write file content to zip: {}", e))
        })?;
    }

    let buffer_cursor = zip.finish().map_err(|e| {
        KsefError::ApplicationError(0, format!("Failed to finish zip creation: {}", e))
    })?;
    let buffer = buffer_cursor.into_inner();

    let hash = sha256(&buffer);
    let size = buffer.len();

    Ok(BatchZipResult {
        content: buffer,
        metadata: FileMetadata {
            hash: hash.to_vec(),
            size,
        },
    })
}

pub fn calculate_invoice_hash(content: &[u8]) -> Vec<u8> {
    sha256(content).to_vec()
}
