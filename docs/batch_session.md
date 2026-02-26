[Polska wersja](batch_session.md) / [English version](batch_session.en.md)

__[Oficjalna Dokumentacja](https://github.com/CIRFMF/ksef-docs/blob/main/sesja-wsadowa.md)__

# Sesja wsadowa

Interfejs sesji wsadowej pozwala przesłać wiele faktur w jednym żądaniu.
Biblioteka wykonuje za Ciebie:

1. utworzenie archiwum ZIP zawierającego wszystkie faktury,
2. opcjonalne podzielenie go na części,
3. zaszyfrowanie każdej części,
4. otwarcie sesji wsadowej u API,
5. przesłanie zaszyfrowanych części,
6. zamknięcie sesji.

Jeżeli potrzebujesz tylko prostego opakowania, użyj `submit_batch`, które
dokonuje całego przepływu i zwraca informacje o wysłanym zbiorze faktur.
Alternatywnie możesz wywołać niskopoziomowe funkcje
`open_batch_session`, `upload_batch_parts` i `close_batch_session`
samodzielnie, jeżeli chcesz mieć większą kontrolę.

### 1. Przygotowanie faktur

Każda faktura musi być zapakowana w strukturę `InvoicePayload` zawierającą
nazwę pliku i surowe bajty XML dokumentu.

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

### 2. Wysłanie wsadu

Wywołaj `submit_batch`, aby wykonać cały przepływ sesji wsadowej.  
Pierwszy argument to odniesienie do wektora faktur przygotowanego powyżej.
Drugi argument (`max_part_size_bytes`) to **opcjonalny** maksymalny rozmiar każdej
części ZIP *przed szyfrowaniem*. Jeśli przekażesz `None`, zastosowany zostanie
domyślny limit 50 * 1024 * 1024 bajtów (≈ 50 MiB). Możesz podać mniejszą wartość
jeśli Twoja aplikacja tego wymaga.

```rust
let result = client
    .submit_batch(&invoices, Some(10 * 1024 * 1024)).await
    .expect("Failed to submit batch");

println!(
    "submitted batch {}, {} parts, {} bytes total",
    result.reference_number, result.number_of_parts, result.total_size_bytes
);
```

Zwrócony `BatchSubmissionResult` zawiera numer referencyjny sesji, liczbę
wysłanych części oraz łączny rozmiar pliku ZIP.

Możesz także pracować bezpośrednio z poszczególnymi elementami składowymi:

```rust
let zip = create_zip(&invoices)?;
let parts = split_zip(&zip.content, 50 * 1024 * 1024); // domyślny podział
let enc = client.generate_encryption_data().await?;
let encrypted = encrypt_zip_parts(&parts, &enc.symmetric_key, &enc.initialization_vector)?;

let open_req = OpenBatchSessionRequestBuilder::new()
    .with_batch_file_info(zip.metadata.size, &zip.metadata.hash)
    .with_encryption(&enc.encrypted_symmetric_key, &enc.initialization_vector)
    .add_file_part(1, encrypted[0].metadata.size, &encrypted[0].metadata.hash)
    // ... dodaj więcej części ...
    .build()?;

let response = client.open_batch_session(open_req).await?;
client.upload_batch_parts(&response, &encrypted).await?;
client.close_batch_session(&response.reference_number).await?;
```
