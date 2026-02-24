Polska Wersja / [English version](online_session.en.md)

# Wysyłka Faktury w Sesji Interaktywnej

__[Oficjalna Dokumentacja](https://github.com/CIRFMF/ksef-docs/blob/main/sesja-interaktywna.md)__

Interfejs sesji interaktywnej pozwala przesłać fakturę w jednej sesji.
Biblioteka wykonuje za Ciebie:

1. wygenerowanie danych szyfrujących,
2. otwarcie sesji interaktywnej u API,
3. zaszyfrowanie i przesłanie faktury,
4. zamknięcie sesji.

Jeżeli potrzebujesz tylko prostego opakowania, użyj `submit_online`, które
dokonuje całego przepływu i zwraca informacje o wysłanej fakturze.
Alternatywnie możesz wywołać niskopoziomowe funkcje
`open_online_session`, `send_invoice` i `close_online_session`
samodzielnie, jeżeli chcesz mieć większą kontrolę.

### 1. Zautomatyzowana wysyłka faktury

```rust
let invoice_xml = /* faktura XML FA(2) lub FA(3) */;

let result = client
    .submit_online(invoice_xml.as_bytes())
    .expect("Failed to submit online session");

println!(
    "Session reference number: {}",
    result.session_reference_number
);

println!(
    "Invoice reference number: {}",
    result.invoice_reference_number
);
```

### 2. Ręczne zarządzanie sesją

#### 2.1. Otwieranie sesji interaktywnej

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

#### 2.2. Wysyłanie faktury

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

#### 2.3. Sprawdzenie statusu faktury

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

#### 2.4. Zamknięcie sesji interaktywnej

```rust
match client.close_online_session(&session_reference_number) {
    Ok(()) => {}
    Err(e) => {
        panic!("Failed to close online session: {:?}", e);
    }
}
```
