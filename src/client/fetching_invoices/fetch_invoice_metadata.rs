use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct FetchInvoiceMetadataRequest {
    pub query: QueryCriteria,
    pub page_offset: Option<i32>,
    pub page_size: Option<i32>,
}

impl FetchInvoiceMetadataRequest {
    pub fn builder() -> FetchInvoiceMetadataRequestBuilder {
        FetchInvoiceMetadataRequestBuilder::new()
    }
}

pub struct FetchInvoiceMetadataRequestBuilder {
    query: Option<QueryCriteria>,
    page_offset: Option<i32>,
    page_size: Option<i32>,
}

impl Default for FetchInvoiceMetadataRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl FetchInvoiceMetadataRequestBuilder {
    pub fn new() -> Self {
        Self {
            query: None,
            page_offset: None,
            page_size: None,
        }
    }

    pub fn query(mut self, query: QueryCriteria) -> Self {
        self.query = Some(query);
        self
    }

    pub fn page_offset(mut self, offset: i32) -> Self {
        self.page_offset = Some(offset);
        self
    }

    pub fn page_size(mut self, size: i32) -> Self {
        self.page_size = Some(size);
        self
    }

    pub fn build(self) -> Result<FetchInvoiceMetadataRequest, &'static str> {
        let query = self.query.ok_or("query is required")?;
        Ok(FetchInvoiceMetadataRequest {
            query,
            page_offset: self.page_offset,
            page_size: self.page_size,
        })
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct QueryCriteria {
    #[serde(rename = "subjectType")]
    pub subject_type: SubjectType,
    #[serde(rename = "dateRange")]
    pub date_range: DateRange,
    #[serde(rename = "ksefNumber", skip_serializing_if = "Option::is_none")]
    pub ksef_number: Option<String>,
    #[serde(rename = "invoiceNumber", skip_serializing_if = "Option::is_none")]
    pub invoice_number: Option<String>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<AmountFilter>,
    #[serde(rename = "sellerNip", skip_serializing_if = "Option::is_none")]
    pub seller_nip: Option<String>,
    #[serde(rename = "buyerIdentifier", skip_serializing_if = "Option::is_none")]
    pub buyer_identifier: Option<BuyerIdentifier>,
    #[serde(rename = "currencyCodes", skip_serializing_if = "Option::is_none")]
    pub currency_codes: Option<Vec<String>>,
    #[serde(rename = "invoicingMode", skip_serializing_if = "Option::is_none")]
    pub invoicing_mode: Option<InvoicingMode>,
    #[serde(rename = "isSelfInvoicing", skip_serializing_if = "Option::is_none")]
    pub is_self_invoicing: Option<bool>,
    #[serde(rename = "formType", skip_serializing_if = "Option::is_none")]
    pub form_type: Option<FormType>,
    #[serde(rename = "invoiceTypes", skip_serializing_if = "Option::is_none")]
    pub invoice_types: Option<Vec<InvoiceType>>,
    #[serde(rename = "hasAttachment", skip_serializing_if = "Option::is_none")]
    pub has_attachment: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct QueryCriteriaBuilder {
    subject_type: Option<SubjectType>,
    date_range: Option<DateRange>,
    ksef_number: Option<String>,
    invoice_number: Option<String>,
    amount: Option<AmountFilter>,
    seller_nip: Option<String>,
    buyer_identifier: Option<BuyerIdentifier>,
    currency_codes: Option<Vec<String>>,
    invoicing_mode: Option<InvoicingMode>,
    is_self_invoicing: Option<bool>,
    form_type: Option<FormType>,
    invoice_types: Option<Vec<InvoiceType>>,
    has_attachment: Option<bool>,
}

impl Default for QueryCriteriaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl QueryCriteriaBuilder {
    pub fn new() -> Self {
        Self {
            subject_type: None,
            date_range: None,
            ksef_number: None,
            invoice_number: None,
            amount: None,
            seller_nip: None,
            buyer_identifier: None,
            currency_codes: None,
            invoicing_mode: None,
            is_self_invoicing: None,
            form_type: None,
            invoice_types: None,
            has_attachment: None,
        }
    }

    pub fn subject_type(mut self, subject: SubjectType) -> Self {
        self.subject_type = Some(subject);
        self
    }

    pub fn date_range(mut self, range: DateRange) -> Self {
        self.date_range = Some(range);
        self
    }

    pub fn ksef_number(mut self, v: impl Into<String>) -> Self {
        self.ksef_number = Some(v.into());
        self
    }

    pub fn invoice_number(mut self, v: impl Into<String>) -> Self {
        self.invoice_number = Some(v.into());
        self
    }

    pub fn amount(mut self, a: AmountFilter) -> Self {
        self.amount = Some(a);
        self
    }

    pub fn seller_nip(mut self, v: impl Into<String>) -> Self {
        self.seller_nip = Some(v.into());
        self
    }

    pub fn buyer_identifier(mut self, identifier: BuyerIdentifier) -> Self {
        self.buyer_identifier = Some(identifier);
        self
    }

    pub fn currency_codes(mut self, codes: Vec<String>) -> Self {
        self.currency_codes = Some(codes);
        self
    }

    pub fn invoicing_mode(mut self, mode: InvoicingMode) -> Self {
        self.invoicing_mode = Some(mode);
        self
    }

    pub fn is_self_invoicing(mut self, flag: bool) -> Self {
        self.is_self_invoicing = Some(flag);
        self
    }

    pub fn form_type(mut self, ft: FormType) -> Self {
        self.form_type = Some(ft);
        self
    }

    pub fn invoice_types(mut self, types: Vec<InvoiceType>) -> Self {
        self.invoice_types = Some(types);
        self
    }

    pub fn has_attachment(mut self, flag: bool) -> Self {
        self.has_attachment = Some(flag);
        self
    }

    pub fn build(self) -> Result<QueryCriteria, &'static str> {
        let subject_type = self.subject_type.ok_or("subject_type is required")?;
        let date_range = self.date_range.ok_or("date_range is required")?;

        Ok(QueryCriteria {
            subject_type,
            date_range,
            ksef_number: self.ksef_number,
            invoice_number: self.invoice_number,
            amount: self.amount,
            seller_nip: self.seller_nip,
            buyer_identifier: self.buyer_identifier,
            currency_codes: self.currency_codes,
            invoicing_mode: self.invoicing_mode,
            is_self_invoicing: self.is_self_invoicing,
            form_type: self.form_type,
            invoice_types: self.invoice_types,
            has_attachment: self.has_attachment,
        })
    }
}

#[derive(Debug, Serialize, Clone)]
pub enum SubjectType {
    #[serde(rename = "Subject1")]
    Subject1,
    #[serde(rename = "Subject2")]
    Subject2,
    #[serde(rename = "Subject3")]
    Subject3,
    #[serde(rename = "SubjectAuthorized")]
    SubjectAuthorized,
}

#[derive(Debug, Serialize, Clone)]
pub struct DateRange {
    #[serde(rename = "dateType")]
    pub date_type: DateType,
    #[serde(rename = "from")]
    pub from: String,
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(
        rename = "restrictToPermanentStorageHwmDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub restrict_to_permanent_storage_hwm_date: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct DateRangeBuilder {
    date_type: Option<DateType>,
    from: Option<String>,
    to: Option<String>,
    restrict_to_permanent_storage_hwm_date: Option<bool>,
}

impl Default for DateRangeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl DateRangeBuilder {
    pub fn new() -> Self {
        Self {
            date_type: None,
            from: None,
            to: None,
            restrict_to_permanent_storage_hwm_date: None,
        }
    }

    pub fn date_type(mut self, dt: DateType) -> Self {
        self.date_type = Some(dt);
        self
    }

    pub fn from(mut self, v: impl Into<String>) -> Self {
        self.from = Some(v.into());
        self
    }

    pub fn to(mut self, v: impl Into<String>) -> Self {
        self.to = Some(v.into());
        self
    }

    pub fn restrict_to_permanent_storage_hwm_date(mut self, flag: bool) -> Self {
        self.restrict_to_permanent_storage_hwm_date = Some(flag);
        self
    }

    pub fn build(self) -> Result<DateRange, &'static str> {
        let date_type = self.date_type.ok_or("date_type is required")?;
        let from = self.from.ok_or("from is required")?;
        Ok(DateRange {
            date_type,
            from,
            to: self.to,
            restrict_to_permanent_storage_hwm_date: self.restrict_to_permanent_storage_hwm_date,
        })
    }
}

#[derive(Debug, Serialize, Clone)]
pub enum DateType {
    #[serde(rename = "Issue")]
    Issue,
    #[serde(rename = "Invoicing")]
    Invoicing,
    #[serde(rename = "PermanentStorage")]
    PermanentStorage,
}

#[derive(Debug, Serialize, Clone)]
pub struct AmountFilter {
    #[serde(rename = "type")]
    pub amount_type: AmountType,
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<f64>,
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<f64>,
}

#[derive(Debug, Serialize, Clone)]
pub enum AmountType {
    #[serde(rename = "Brutto")]
    Brutto,
    #[serde(rename = "Netto")]
    Netto,
    #[serde(rename = "Vat")]
    Vat,
}

#[derive(Debug, Serialize, Clone)]
pub struct BuyerIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: BuyerIdentifierType,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BuyerIdentifierType {
    #[serde(rename = "Nip")]
    Nip,
    #[serde(rename = "VatUe")]
    VatUe,
    #[serde(rename = "Other")]
    Other,
    #[serde(rename = "None")]
    None,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum InvoicingMode {
    #[serde(rename = "Online")]
    Online,
    #[serde(rename = "Offline")]
    Offline,
}

#[derive(Debug, Serialize, Clone)]
pub enum FormType {
    #[serde(rename = "FA")]
    FA,
    #[serde(rename = "PEF")]
    PEF,
    #[serde(rename = "RR")]
    RR,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum InvoiceType {
    #[serde(rename = "Vat")]
    Vat,
    #[serde(rename = "Zal")]
    Zal,
    #[serde(rename = "Kor")]
    Kor,
    #[serde(rename = "Roz")]
    Roz,
    #[serde(rename = "Upr")]
    Upr,
    #[serde(rename = "KorZal")]
    KorZal,
    #[serde(rename = "KorRoz")]
    KorRoz,
    #[serde(rename = "VatPef")]
    VatPef,
    #[serde(rename = "VatPefSp")]
    VatPefSp,
    #[serde(rename = "KorPef")]
    KorPef,
    #[serde(rename = "VatRr")]
    VatRr,
    #[serde(rename = "KorVatRr")]
    KorVatRr,
}

#[derive(Debug, Deserialize)]
pub struct FetchInvoiceMetadataResponse {
    #[serde(rename = "hasMore")]
    pub has_more: bool,
    #[serde(rename = "isTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "permanentStorageHwmDate")]
    pub permanent_storage_hwm_date: Option<String>,
    #[serde(rename = "invoices")]
    pub invoices: Vec<InvoiceMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct InvoiceMetadata {
    #[serde(rename = "ksefNumber")]
    pub ksef_number: String,
    #[serde(rename = "invoiceNumber")]
    pub invoice_number: String,
    #[serde(rename = "issueDate")]
    pub issue_date: String,
    #[serde(rename = "invoicingDate")]
    pub invoicing_date: String,
    #[serde(rename = "acquisitionDate")]
    pub acquisition_date: Option<String>,
    #[serde(rename = "permanentStorageDate")]
    pub permanent_storage_date: Option<String>,
    #[serde(rename = "seller")]
    pub seller: SellerMetadata,
    #[serde(rename = "buyer")]
    pub buyer: BuyerMetadata,
    #[serde(rename = "netAmount")]
    pub net_amount: Option<f64>,
    #[serde(rename = "grossAmount")]
    pub gross_amount: Option<f64>,
    #[serde(rename = "vatAmount")]
    pub vat_amount: Option<f64>,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "invoicingMode")]
    pub invoicing_mode: InvoicingMode,
    #[serde(rename = "invoiceType")]
    pub invoice_type: InvoiceType,
    #[serde(rename = "formCode")]
    pub form_code: InvoiceFormCode,
    #[serde(rename = "isSelfInvoicing")]
    pub is_self_invoicing: bool,
    #[serde(rename = "hasAttachment")]
    pub has_attachment: bool,
    #[serde(rename = "invoiceHash")]
    pub invoice_hash: String,
    #[serde(rename = "hashOfCorrectedInvoice")]
    pub hash_of_corrected_invoice: Option<String>,
    #[serde(rename = "thirdSubjects")]
    pub third_subjects: Option<Vec<ThirdSubjectMetadata>>,
    #[serde(rename = "authorizedSubject")]
    pub authorized_subject: Option<AuthorizedSubjectMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct SellerMetadata {
    #[serde(rename = "nip")]
    pub nip: String,
    #[serde(rename = "name")]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BuyerMetadata {
    #[serde(rename = "identifier")]
    pub identifier: BuyerIdentifierMetadata,
    #[serde(rename = "name")]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BuyerIdentifierMetadata {
    #[serde(rename = "type")]
    pub identifier_type: BuyerIdentifierType,
    #[serde(rename = "value")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InvoiceFormCode {
    #[serde(rename = "systemCode")]
    pub system_code: String,
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct ThirdSubjectMetadata {
    #[serde(rename = "identifier")]
    pub identifier: ThirdSubjectIdentifier,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "role")]
    pub role: i32,
}

#[derive(Debug, Deserialize)]
pub struct ThirdSubjectIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: String, // Enum: "Nip" "InternalId" "VatUe" "Other" "None"
    #[serde(rename = "value")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AuthorizedSubjectMetadata {
    #[serde(rename = "nip")]
    pub nip: String,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "role")]
    pub role: i32,
}

pub async fn fetch_invoice_metadata(
    client: &KsefClient,
    request: FetchInvoiceMetadataRequest,
) -> Result<FetchInvoiceMetadataResponse, KsefError> {
    let url = client.url_for(routes::INVOICES_QUERY_METADATA_PATH);
    let http = &client.client;

    let token = KsefClient::secret_str(&client.access_token.access_token);
    if token.is_empty() {
        return Err(KsefError::ApplicationError(
            0,
            "No access token available. Please authenticate and redeem token first.".to_string(),
        ));
    }

    let mut req = http
        .post(&url)
        .header("Accept", "application/json")
        .bearer_auth(token);

    if let Some(offset) = request.page_offset {
        req = req.query(&[("pageOffset", offset)]);
    }
    if let Some(size) = request.page_size {
        req = req.query(&[("pageSize", size)]);
    }

    let resp = req.json(&request.query).send().await?;

    let status = resp.status();

    if !status.is_success() {
        let code = status.as_u16();
        let body = resp.text().await.unwrap_or_default();
        return Err(KsefError::from_api_response(code, body));
    }

    let parsed: FetchInvoiceMetadataResponse = resp.json().await?;

    Ok(parsed)
}
