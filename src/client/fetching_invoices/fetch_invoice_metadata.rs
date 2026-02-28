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

    let token = &client.access_token.access_token;
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
        return Err(KsefError::ApiError(code, body));
    }

    let parsed: FetchInvoiceMetadataResponse = resp.json().await?;

    Ok(parsed)
}
