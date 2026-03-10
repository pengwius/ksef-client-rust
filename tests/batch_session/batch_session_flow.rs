use crate::common;
use ksef_client::prelude::*;

use ksef_client::invoices::InvoicePayload;
use ksef_client::sessions::{
    OpenBatchSessionRequestBuilder, calculate_invoice_hash, create_zip, encrypt_zip_parts,
    split_zip,
};
use openssl::symm::{Cipher, decrypt};
use std::io::Cursor;
use zip::ZipArchive;

#[tokio::test]
async fn test_batch_zip_flow() {
    let issuer_nip = "5261234567";
    let invoice_xml_1: String = common::generate_fa2_invoice(issuer_nip).await;
    let invoice_xml_2: String = common::generate_fa2_invoice(issuer_nip).await;

    let invoices = vec![
        InvoicePayload {
            filename: "invoice1.xml".to_string(),
            content: invoice_xml_1.as_bytes().to_vec(),
        },
        InvoicePayload {
            filename: "invoice2.xml".to_string(),
            content: invoice_xml_2.as_bytes().to_vec(),
        },
    ];

    let hash_1 = calculate_invoice_hash(invoice_xml_1.as_bytes());
    assert_eq!(hash_1.len(), 32);

    let zip_result = create_zip(&invoices).expect("Failed to create ZIP");

    assert!(!zip_result.content.is_empty());
    assert_eq!(zip_result.metadata.size, zip_result.content.len());
    assert_eq!(zip_result.metadata.hash.len(), 32);

    let reader = Cursor::new(&zip_result.content);
    let mut archive = ZipArchive::new(reader).expect("Failed to read created ZIP");
    assert_eq!(archive.len(), 2);

    let file1 = archive
        .by_name("invoice1.xml")
        .expect("invoice1.xml missing");
    assert!(file1.size() > 0);
    drop(file1);

    let file2 = archive
        .by_name("invoice2.xml")
        .expect("invoice2.xml missing");
    assert!(file2.size() > 0);
    drop(file2);

    let zip_size = zip_result.content.len();
    assert!(zip_size > 100, "ZIP too small for meaningful split test");

    let max_part_size = zip_size / 2;
    let parts = split_zip(&zip_result.content, max_part_size);

    assert!(parts.len() >= 2, "Should be split into at least 2 parts");

    let reassembled: Vec<u8> = parts.clone().into_iter().flatten().collect();
    assert_eq!(
        reassembled, zip_result.content,
        "Reassembled parts should match original ZIP"
    );

    let key = vec![1u8; 32];
    let iv = vec![2u8; 16];

    let encrypted_parts = encrypt_zip_parts(&parts, &key, &iv).expect("Failed to encrypt parts");

    assert_eq!(encrypted_parts.len(), parts.len());

    for (i, enc_part) in encrypted_parts.iter().enumerate() {
        assert_eq!(enc_part.ordinal_number, i + 1);
        assert!(!enc_part.content.is_empty());
        assert_ne!(
            enc_part.content, parts[i],
            "Encrypted content should differ from plaintext"
        );
        assert_eq!(enc_part.metadata.size, enc_part.content.len());

        let decrypted = decrypt(
            Cipher::aes_256_cbc(),
            key.as_slice(),
            Some(iv.as_slice()),
            &enc_part.content,
        )
        .expect("Failed to decrypt part");

        assert_eq!(
            decrypted, parts[i],
            "Decrypted content should match original part"
        );
    }
}

#[tokio::test]
async fn test_batch_session_initialization() {
    let client: ksef_client::KsefClient = common::authorize_client().await;

    let issuer_nip = "5261234567";
    let invoice_xml: String = common::generate_fa2_invoice(issuer_nip).await;

    let invoices = vec![InvoicePayload {
        filename: "invoice.xml".to_string(),
        content: invoice_xml.as_bytes().to_vec(),
    }];

    let zip_result = create_zip(&invoices).expect("Failed to create ZIP");

    let encryption_data = client
        .generate_encryption_data()
        .await
        .expect("Failed to generate encryption data");

    let parts = vec![zip_result.content.clone()];
    let encrypted_parts = encrypt_zip_parts(
        &parts,
        &encryption_data.symmetric_key,
        &encryption_data.initialization_vector,
    )
    .expect("Failed to encrypt parts");

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

    let request = builder
        .build()
        .expect("Failed to build OpenBatchSessionRequest");

    let response = client
        .open_batch_session(request)
        .await
        .expect("Failed to open batch session");

    println!("Opened batch session: {:?}", response);

    assert!(
        !response.reference_number.is_empty(),
        "Session reference number should not be empty"
    );
    assert!(
        !response.part_upload_requests.is_empty(),
        "Should have upload requests"
    );
    assert_eq!(
        response.part_upload_requests.len(),
        1,
        "Should have 1 upload request"
    );

    match client.upload_batch_parts(&response, &encrypted_parts).await {
        Ok(()) => {}
        Err(e) => {
            panic!("Failed to upload batch parts: {:?}", e);
        }
    }

    match client.close_batch_session(&response.reference_number).await {
        Ok(()) => {}
        Err(e) => {
            panic!("Failed to close batch session: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_submit_batch_automated() {
    let client: ksef_client::KsefClient = common::authorize_client().await;

    let issuer_nip = "5261234567";
    let invoice_xml_1: String = common::generate_fa2_invoice(issuer_nip).await;
    let invoice_xml_2: String = common::generate_fa2_invoice(issuer_nip).await;

    let invoices = vec![
        InvoicePayload {
            filename: "invoice1.xml".to_string(),
            content: invoice_xml_1.as_bytes().to_vec(),
        },
        InvoicePayload {
            filename: "invoice2.xml".to_string(),
            content: invoice_xml_2.as_bytes().to_vec(),
        },
    ];

    let result = client
        .submit_batch(&invoices, Some(10 * 1024 * 1024))
        .await
        .expect("Failed to submit batch");

    println!("Automated batch submission successful: {:?}", result);

    assert!(
        !result.reference_number.is_empty(),
        "Reference number should not be empty"
    );
    assert!(result.number_of_parts > 0, "Should have at least 1 part");
    assert!(
        result.total_size_bytes > 0,
        "Total size should be greater than 0"
    );
}
