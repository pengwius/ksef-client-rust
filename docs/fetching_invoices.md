Polska wersja / [English version](fetching_invoices.en.md)

__[Oficjalna Dokumentacja](https://github.com/CIRFMF/ksef-docs/blob/main/pobieranie-faktur/pobieranie-faktur.md)__

# Pobieranie faktur

### 1. Pobieranie metadanych faktur

```rust
let now = chrono::Utc::now();
let start_of_day = now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
let from_date = start_of_day.to_rfc3339();
let to_date = (now + chrono::Duration::hours(1)).to_rfc3339();

let date_range = DateRangeBuilder::new()
    .date_type(DateType::Invoicing)
    .from(from_date.clone())
    .to(to_date.clone())
    .build()?;

let query = QueryCriteriaBuilder::new()
    .subject_type(SubjectType::Subject1)
    .date_range(date_range)
    .build()?;

let query_req = FetchInvoiceMetadataRequestBuilder::new()
    .query(query)
    .page_offset(0)
    .page_size(100)
    .build()?;

let metadata_resp = client.fetch_invoice_metadata(query_req.clone()).await?;
println!("Total results: {}", metadata_resp.total_results);
```

- `page_size` i `page_offset` kontrolują paginację. Zwróć uwagę na limity API i rozmiar wyników.

### `DateRangeBuilder`

- `date_type(DateType)` — wymagane. Typ daty, dostępne wartości: `Issue`, `Invoicing`, `PermanentStorage`.
- `from(String)` — wymagane. Początek zakresu w formacie RFC3339 (np. "2024-02-01T00:00:00Z").
- `to(String)` — opcjonalne. Koniec zakresu (RFC3339). Jeżeli puste, oznacza „do teraz”.
- `restrict_to_permanent_storage_hwm_date(bool)` — opcjonalne. Flaga, która wskazuje czy zakres ma być ograniczony do wartości HWM (przydatne w scenariuszach przyrostowych).

```rust
let date_range = DateRangeBuilder::new()
    .date_type(DateType::Invoicing)
    .from("2024-02-01T00:00:00Z")
    .to("2024-02-02T00:00:00Z")
    .restrict_to_permanent_storage_hwm_date(false)
    .build()?;
```

- `DateType` wybierasz w zależności od tego, po jakim znaczeniu dat chcesz filtrować (data wystawienia, data księgowania itp.).
- `from` i `to` powinny być poprawnie zserializowane do RFC3339 — warto użyć `chrono` do generacji tych wartości.


### `QueryCriteriaBuilder`

Wymagane:
- `subject_type(SubjectType)` — wymagane. Typ podmiotu (np. `Subject1`, `Subject2`, `Subject3`, `SubjectAuthorized`).
- `date_range(DateRange)` — wymagane. Zakres dat utworzony z `DateRangeBuilder`.

Opcjonalne pola:
- `ksef_number(String)` — numer KSeF.
- `invoice_number(String)` — numer faktury nadany przez wystawcę.
- `amount(AmountFilter)` — filtr po kwocie (z typem: Brutto/Netto/Vat oraz opcjonalnym zakresem from/to).
- `seller_nip(String)` — NIP sprzedawcy.
- `buyer_identifier(BuyerIdentifier)` — typ i opcjonalna wartość identyfikatora nabywcy.
- `currency_codes(Vec<String>)` — lista kodów walut (np. ["PLN", "EUR"]).
- `invoicing_mode(InvoicingMode)` — `Online` lub `Offline`.
- `is_self_invoicing(bool)` — filtr faktur samofakturowanych.
- `form_type(FormType)` — typ dokumentu (np. `FA`, `PEF`, `RR`).
- `invoice_types(Vec<InvoiceType>)` — lista typów faktur (np. `Vat`, `Kor`).
- `has_attachment(bool)` — czy faktura ma załącznik.

```rust
let query = QueryCriteriaBuilder::new()
    .subject_type(SubjectType::Subject1)
    .date_range(date_range)
    .seller_nip("1234567890")
    .currency_codes(vec!["PLN".to_string()])
    .invoicing_mode(InvoicingMode::Offline)
    .has_attachment(false)
    .build()?;
```

### `FetchInvoiceMetadataRequestBuilder`

- `query(QueryCriteria)` — wymagane. Kryteria wyszukiwania (zbudowane przez `QueryCriteriaBuilder`).
- `page_offset(i32)` — opcjonalne. Offset paginacji (domyślnie 0).
- `page_size(i32)` — opcjonalne. Rozmiar strony (ile wyników na jedną stronę).

```rust
let req = FetchInvoiceMetadataRequestBuilder::new()
    .query(query)
    .page_offset(0)
    .page_size(100)
    .build()?;
```

### 2. Pobieranie pojedynczej faktury (XML)

Jeżeli znasz `ksefNumber` (numer KSeF) faktury, możesz pobrać jej treść

```rust
let ksef_number = /* numer KSeF faktury jako String */;

let invoice = client.fetch_invoice(&ksef_number).await?;
let xml_bytes = invoice.content; // Vec<u8>
let xml_text = String::from_utf8_lossy(&xml_bytes);
println!("Invoice XML:\n{}", xml_text);
```

### 3. Eksportowanie faktur

Eksport to flow, które:
1. Generuje dane szyfrujące (symetryczny klucz + IV i zaszyfrowany klucz wysyłany do KSeF),
2. Uruchamia eksport (`POST /v2/invoices/exports`),
3. Polluje status eksportu (`GET /v2/invoices/exports/{ref}`),
4. Po sukcesie pobiera część/ części paczki, odszyfrowuje je i zwraca zawartość.

Przykład wywołania helpera `export_invoices`:

```rust
let export_result = client
    .export_invoices(query_req.query.clone())
    .await?; // Zwraca strukturyzowany wynik: status + odszyfrowane części

let exported_parts = export_result.parts;
println!("Exported {} parts", exported_parts.len());

let output_dir = std::path::Path::new("exported_packages");
std::fs::create_dir_all(&output_dir)?;

for part in exported_parts.iter() {
    println!("Processing part: {}", part.metadata.part_name);

    // Fragment A: szybki zapis odszyfrowanej części jako surowy plik .zip
    let part_file_path = output_dir.join(&part.metadata.part_name);
    std::fs::write(&part_file_path, &part.content)?; // part.content jest odszyfrowane
    println!("Saved exported part to {:?}", part_file_path);

    // Fragment B: otwarcie ZIP w pamięci i przetworzenie plików wewnątrz
    let cursor = std::io::Cursor::new(&part.content);
    let mut archive = zip::ZipArchive::new(cursor)?;
    let part_dir = output_dir.join(part.metadata.part_name.replace("/", "_"));
    std::fs::create_dir_all(&part_dir)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_name = file.name().to_string();
        let mut contents = Vec::new();
        use std::io::Read;
        file.read_to_end(&mut contents)?;
        let target_path = part_dir.join(&file_name);
        std::fs::write(&target_path, &contents)?;
        println!("Extracted {} -> {:?}", file_name, target_path);
    }
}
```

- `Fragment A` — szybko zapisujesz dokładnie te bajty, które zwrócił klient.
- `Fragment B` — od razu rozpakowujesz ZIP i przetwarzasz zawartość, np. odczytujesz pliki XML i `_metadata.json`.

### 4. Przyrostowe pobieranie (HWM — High Water Mark)

Dla synchronizacji długoterminowej zalecane jest pobieranie przyrostowe. Idea:
- Utrzymuj stan kontynuacji osobno dla każdego `SubjectType` (np. `Subject1`, `Subject2`).
- Pobieraj okno czasowe (window) zaczynając od punktu startowego (domyślnie ostatni znany HWM).
- Po każdej udanej paczce aktualizuj punkt kontynuacji:
  - jeżeli `isTruncated == true` → użyj `lastPermanentStorageDate` jako kolejnego punktu startowego,
  - w przeciwnym razie → użyj `permanentStorageHwmDate`.

Przykład użycia helpera:

```rust
let mut state = IncrementalFetchState::new();
let subject_types = vec![SubjectType::Subject1];
let default_start = chrono::Utc::now() - chrono::Duration::days(1);
let window_end = Some(chrono::Utc::now() + chrono::Duration::days(1));

let fetched_invoices = client
    .export_invoices_incrementally(&mut state, subject_types, window_end, default_start)
    .await?; // Zwraca wektor unikatowych faktur z metadanymi i treścią

println!("Incrementally fetched {} invoices", fetched_invoices.len());

for invoice in &fetched_invoices {
    println!(
        "Fetched invoice: {} (KSeF: {})",
        invoice.metadata.invoice_number, invoice.metadata.ksef_number
    );
}
```

- Jeżeli eksport był `isTruncated`, trzeba powtórzyć kolejne wywołania eksportu, przesuwając punkt startowy zgodnie z regułą HWM, aż do momentu, gdy wszystkie dane w oknie zostaną pobrane.
