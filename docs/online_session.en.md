[Polska Wersja](online_session.md) / English version

# Sending an Invoice in an Interactive Session

__[Official Documentation](https://github.com/CIRFMF/ksef-docs/blob/main/sesja-interaktywna.md)__

### 1. Opening an interactive session

```rust
// Generate encryption data for the interactive session
let encryption_data = client
    .generate_encryption_data()
    .expect("Failed to generate encryption data");

let request = OpenOnlineSessionRequestBuilder::new()
    .with_encryption(
        &encryption_data.encrypted_symmetric_key,
        &encryption_data.initialization_vector,
    )
    .build()
    .expect("Failed to build OpenOnlineSessionRequest");

let session_reference_number = match client.open_online_session(request) {
    Ok(response) => response.reference_number,
    Err(error) => {
        eprintln!("Failed to open online session: {}", error);
        return;
    }
};
```

### 2. Sending an invoice

```rust
let issuer_nip = "5264567890"; // Invoice issuer identifier
let invoice_xml = /* FA(2) or FA(3) XML invoice */;

let invoice_reference_number = match client.send_invoice(
    &session_reference_number,
    invoice_xml.as_bytes(),
    &encryption_data,
) {
    Ok(response) => response.reference_number,
    Err(error) => {
        eprintln!("Failed to send invoice: {}", error);
        return;
    }
};
```

### 3. Checking invoice status

```rust
let status = client
    .get_invoice_status(&session_reference_number, &invoice_reference_number)
    .expect("Failed to get invoice status");

if status.invoice_status.code != 200 {
    eprintln!("Invoice processing failed with status: {} - {}", 
        status.invoice_status.code, 
        status.invoice_status.description
    );
    return;
}
```

### 4. Closing the interactive session

```rust
match client.close_online_session(&session_reference_number) {
    Ok(()) => {}
    Err(e) => {
        panic!("Failed to close online session: {:?}", e);
    }
}
```
