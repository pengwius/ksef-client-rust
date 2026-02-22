Polska Wersja / [English version](online_session.en.md)

# Wysyłka Faktury w Sesji Interaktywnej

__[Oficjalna Dokumentacja](https://github.com/CIRFMF/ksef-docs/blob/main/sesja-interaktywna.md)__

### 1. Otwieranie sesji interaktywnej

```rust
// Generowanie danych szyfrujących dla sesji interaktywnej
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

### 2. Wysyłanie faktury

```rust
let issuer_nip = "5264567890"; // Identyfikator wystawcy faktury
let invoice_xml = /* faktura XML FA(2) lub FA(3) */;

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

### 3. Sprawdzenie statusu faktury

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

### 4. Zamknięcie sesji interaktywnej

```rust
match client.close_online_session(&session_reference_number) {
    Ok(()) => {}
    Err(e) => {
        panic!("Failed to close online session: {:?}", e);
    }
}
```
