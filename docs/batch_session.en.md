[Polska wersja](batch_session.md) / English version

__[Official Documentation](https://github.com/CIRFMF/ksef-docs/blob/main/sesja-wsadowa.md)__

# Batch session

The batch session API lets you submit multiple invoices in a single operation.
The library takes care of:

1. creating a ZIP archive containing all invoices,
2. optionally splitting it into parts,
3. encrypting each part,
4. opening a batch session with the API,
5. uploading the encrypted parts,
6. closing the session.

If you only need a simple wrapper, use `submit_batch` which performs the
entire flow and returns information about the submitted set of invoices.
Alternatively you can call the lower‑level functions
`open_batch_session`, `upload_batch_parts` and `close_batch_session` yourself
for more granular control.

### 1. Prepare the invoices

Each invoice must be wrapped in an `InvoicePayload` struct containing the
filename and the raw XML bytes of the invoice.

```rust
let invoice_xml_1 = /* invoice 1 */;
let invoice_xml_2 = /* invoice 2 */;

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
```

### 2. Submit the batch

Call `submit_batch` to perform the whole batch‑session flow.  
The first argument is a reference to the invoices vector prepared above.
The second argument (`max_part_size_bytes`) is an **optional** maximum size for
each ZIP part *before encryption*. When `None` is passed, a default limit of
50 * 1024 * 1024 bytes (≈ 50 MiB) is used. You can specify a smaller size if
your application requires it.

```rust
let result = client
    .submit_batch(&invoices, Some(10 * 1024 * 1024)).await
    .expect("Failed to submit batch");

println!(
    "submitted batch {}, {} parts, {} bytes total",
    result.reference_number, result.number_of_parts, result.total_size_bytes
);
```

The returned `BatchSubmissionResult` contains the session reference number, the
number of uploaded parts and the total size of the ZIP file.

You can also work with the individual building blocks directly:

```rust
let zip = create_zip(&invoices)?;
let parts = split_zip(&zip.content, 50 * 1024 * 1024); // default split
let enc = client.generate_encryption_data().await?;
let encrypted = encrypt_zip_parts(&parts, &enc.symmetric_key, &enc.initialization_vector)?;

let open_req = OpenBatchSessionRequestBuilder::new()
    .with_batch_file_info(zip.metadata.size, &zip.metadata.hash)
    .with_encryption(&enc.encrypted_symmetric_key, &enc.initialization_vector)
    .add_file_part(1, encrypted[0].metadata.size, &encrypted[0].metadata.hash)
    // ... add more parts ...
    .build()?;

let response = client.open_batch_session(open_req).await?;
client.upload_batch_parts(&response, &encrypted).await?;
client.close_batch_session(&response.reference_number).await?;
```
