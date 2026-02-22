mod common;

use ksef_client::{
    InvoicePayload, calculate_invoice_hash, create_zip, encrypt_zip_parts, split_zip,
};
use openssl::symm::{Cipher, decrypt};
use std::io::Cursor;
use zip::ZipArchive;

#[test]
fn test_batch_zip_flow() {
    // 1. Prepare invoice payloads
    let issuer_nip = "5264567890";
    let invoice_xml_1 = common::generate_fa2_invoice(issuer_nip);
    let invoice_xml_2 = common::generate_fa2_invoice(issuer_nip);

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

    // Verify hash calculation (correlation logic)
    let hash_1 = calculate_invoice_hash(invoice_xml_1.as_bytes());
    assert_eq!(hash_1.len(), 32);

    // 2. Create ZIP
    let zip_result = create_zip(&invoices).expect("Failed to create ZIP");

    assert!(!zip_result.content.is_empty());
    assert_eq!(zip_result.metadata.size, zip_result.content.len());
    assert_eq!(zip_result.metadata.hash.len(), 32);

    // Verify ZIP content structure
    let reader = Cursor::new(&zip_result.content);
    let mut archive = ZipArchive::new(reader).expect("Failed to read created ZIP");
    assert_eq!(archive.len(), 2);

    let file1 = archive
        .by_name("invoice1.xml")
        .expect("invoice1.xml missing");
    assert!(file1.size() > 0);
    drop(file1); // Release borrow

    let file2 = archive
        .by_name("invoice2.xml")
        .expect("invoice2.xml missing");
    assert!(file2.size() > 0);
    drop(file2);

    // 3. Split ZIP
    // Force splitting by using a small max size.
    // Let's assume the zip is at least a few hundred bytes.
    let zip_size = zip_result.content.len();
    assert!(zip_size > 100, "ZIP too small for meaningful split test");

    // Split into roughly 3 parts
    let max_part_size = zip_size / 2;
    let parts = split_zip(&zip_result.content, max_part_size);

    assert!(parts.len() >= 2, "Should be split into at least 2 parts");

    let reassembled: Vec<u8> = parts.clone().into_iter().flatten().collect();
    assert_eq!(
        reassembled, zip_result.content,
        "Reassembled parts should match original ZIP"
    );

    // 4. Encrypt parts
    // Generate a random key and IV for testing
    let key = vec![1u8; 32]; // 256 bits
    let iv = vec![2u8; 16]; // 128 bits

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

        // Decrypt to verify correctness
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
