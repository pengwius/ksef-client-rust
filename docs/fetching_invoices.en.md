[Polska wersja](fetching_invoices.md) / English version

__[Official Documentation](https://github.com/CIRFMF/ksef-docs/blob/main/pobieranie-faktur/pobieranie-faktur.md)__

# Fetching invoices

### 1. Fetching invoice metadata

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

- `page_size` and `page_offset` control pagination. Pay attention to API limits and the size of result sets.

### `DateRangeBuilder`

- `date_type(DateType)` — required. Which date field to use for filtering. Available values: `Issue`, `Invoicing`, `PermanentStorage`.
- `from(String)` — required. Start of the range in RFC3339 format (e.g. "2024-02-01T00:00:00Z").
- `to(String)` — optional. End of the range (RFC3339). If omitted, treated as open-ended (up to now).
- `restrict_to_permanent_storage_hwm_date(bool)` — optional. If set, restricts the range in relation to permanent storage HWM (useful for incremental flows).

```rust
let date_range = DateRangeBuilder::new()
    .date_type(DateType::Invoicing)
    .from("2024-02-01T00:00:00Z")
    .to("2024-02-02T00:00:00Z")
    .restrict_to_permanent_storage_hwm_date(false)
    .build()?;
```

- Choose `DateType` based on the semantics you need (issue date vs invoicing date vs permanent storage).
- Use `chrono` to produce RFC3339 strings to avoid formatting errors.

### `QueryCriteriaBuilder`

Required:
- `subject_type(SubjectType)` — required. Which subject type to query (e.g. `Subject1`, `Subject2`, `Subject3`, `SubjectAuthorized`).
- `date_range(DateRange)` — required. Built with `DateRangeBuilder`.

Optional fields:
- `ksef_number(String)` — filter by exact KSeF number.
- `invoice_number(String)` — supplier invoice number.
- `amount(AmountFilter)` — amount filter (with `AmountType`: Brutto/Netto/Vat and optional `from`/`to`).
- `seller_nip(String)` — seller NIP.
- `buyer_identifier(BuyerIdentifier)` — buyer identifier type and optional value.
- `currency_codes(Vec<String>)` — currency codes, e.g. `["PLN", "EUR"]`.
- `invoicing_mode(InvoicingMode)` — `Online` or `Offline`.
- `is_self_invoicing(bool)` — filter self-invoiced documents.
- `form_type(FormType)` — document form (e.g. `FA`, `PEF`, `RR`).
- `invoice_types(Vec<InvoiceType>)` — list of invoice kinds (e.g. `Vat`, `Kor`).
- `has_attachment(bool)` — whether the invoice has an attachment.

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

- Combine only meaningful filters; unnecessary filters may undesirably narrow results.
- `BuyerIdentifier` contains `identifier_type: BuyerIdentifierType` and optional `value`.

### `FetchInvoiceMetadataRequestBuilder`

- `query(QueryCriteria)` — required. Criteria built by `QueryCriteriaBuilder`.
- `page_offset(i32)` — optional. Pagination offset (default 0).
- `page_size(i32)` — optional. Page size (results per page).

```rust
let req = FetchInvoiceMetadataRequestBuilder::new()
    .query(query)
    .page_offset(0)
    .page_size(100)
    .build()?;
```

### 2. Fetching a single invoice (XML)

If you know the invoice `ksefNumber` you can fetch its content:

```rust
use ksef_client::types::KsefNumber;

let ksef_number_str = /* KSeF invoice number as String */;
let ksef_number = KsefNumber::new(ksef_number_str);

let invoice = client.fetch_invoice(ksef_number).await?;
let xml_bytes = invoice.content; // Vec<u8>
let xml_text = String::from_utf8_lossy(&xml_bytes);
println!("Invoice XML:\n{}", xml_text);
```

- Note: Depending on configuration, the API might transfer encrypted bytes; the client typically returns decrypted bytes in `content`.

### 3. Exporting invoices

Export flow:
1. Generate encryption data (symmetric key + IV, with the symmetric key encrypted for the server),
2. Start export (`POST /v2/invoices/exports`),
3. Poll export status (`GET /v2/invoices/exports/{ref}`),
4. On success download package part(s), decrypt them locally and return contents.

Example using the `export_invoices` helper:

```rust
let export_result = client
    .export_invoices(query_req.query.clone())
    .await?; // Returns structured result: status + decrypted parts

let exported_parts = export_result.parts;
println!("Exported {} parts", exported_parts.len());

let output_dir = std::path::Path::new("exported_packages");
std::fs::create_dir_all(&output_dir)?;

for part in exported_parts.iter() {
    println!("Processing part: {}", part.metadata.part_name);

    // Fragment A: quickly save the decrypted part as a raw .zip file
    let part_file_path = output_dir.join(&part.metadata.part_name);
    std::fs::write(&part_file_path, &part.content)?; // part.content is decrypted
    println!("Saved exported part to {:?}", part_file_path);

    // Fragment B: open the ZIP in memory and process inner files
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

- Fragment A — save the exact decrypted bytes returned by the client (good for archival).
- Fragment B — unzip and process inner files immediately (XMLs, `_metadata.json`).

### 4. Incremental fetching (HWM — High Water Mark)

For long-term synchronization it is recommended to use incremental fetching:
- Keep continuation state per `SubjectType` (e.g. `Subject1`, `Subject2`).
- Fetch a time window starting from the start point (default: last known HWM).
- After processing a package, update the continuation point:
  - if `isTruncated == true` → use `lastPermanentStorageDate` as the next start,
  - otherwise → use `permanentStorageHwmDate`.

Example using the helper:

```rust
let mut state = IncrementalFetchState::new();
let subject_types = vec![SubjectType::Subject1];
let default_start = chrono::Utc::now() - chrono::Duration::days(1);
let window_end = Some(chrono::Utc::now() + chrono::Duration::days(1));

let fetched_invoices = client
    .export_invoices_incrementally(&mut state, subject_types, window_end, default_start)
    .await?; // Returns a vector of unique invoices with metadata and content

println!("Incrementally fetched {} invoices", fetched_invoices.len());

for invoice in &fetched_invoices {
    println!(
        "Fetched invoice: {} (KSeF: {})",
        invoice.metadata.invoice_number, invoice.metadata.ksef_number
    );
}
```

- If an export response has `isTruncated`, repeat subsequent exports advancing the start point according to HWM rules until the full window is fetched.
