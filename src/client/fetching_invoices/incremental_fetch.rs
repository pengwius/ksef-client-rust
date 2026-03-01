use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::fetching_invoices::export_invoices::{ExportResult, InvoicePackageMetadata};
use crate::client::fetching_invoices::fetch_invoice_metadata::{
    DateRangeBuilder, DateType, InvoiceMetadata, QueryCriteriaBuilder, SubjectType,
};
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use std::io::Read;

#[derive(Debug, Clone)]
pub struct IncrementalFetchState {
    pub continuation_points: HashMap<String, DateTime<Utc>>,
}

impl IncrementalFetchState {
    pub fn new() -> Self {
        Self {
            continuation_points: HashMap::new(),
        }
    }

    pub fn get_start_date(
        &self,
        subject_type: &SubjectType,
        default_start: DateTime<Utc>,
    ) -> DateTime<Utc> {
        let key = format!("{:?}", subject_type);
        self.continuation_points
            .get(&key)
            .cloned()
            .unwrap_or(default_start)
    }

    pub fn update_continuation_point(
        &mut self,
        subject_type: &SubjectType,
        export_result: &ExportResult,
    ) {
        let key = format!("{:?}", subject_type);
        let package = if let Some(pkg) = &export_result.status.package {
            pkg
        } else {
            return;
        };

        if package.is_truncated {
            if let Some(last_date_str) = &package.last_permanent_storage_date {
                if let Ok(date) = DateTime::parse_from_rfc3339(last_date_str) {
                    self.continuation_points
                        .insert(key, date.with_timezone(&Utc));
                }
            }
        } else if let Some(hwm_date_str) = &package.permanent_storage_hwm_date {
            if let Ok(date) = DateTime::parse_from_rfc3339(hwm_date_str) {
                self.continuation_points
                    .insert(key, date.with_timezone(&Utc));
            }
        } else {
        }
    }
}

#[derive(Debug)]
pub struct FetchedInvoice {
    pub metadata: InvoiceMetadata,
    pub content: String,
}

pub async fn fetch_invoices_incrementally(
    client: &KsefClient,
    state: &mut IncrementalFetchState,
    subject_types: Vec<SubjectType>,
    window_end: Option<DateTime<Utc>>,
    default_start: DateTime<Utc>,
) -> Result<Vec<FetchedInvoice>, KsefError> {
    let mut all_fetched_invoices = Vec::new();
    let mut processed_ksef_numbers = HashSet::new();

    for subject_type in subject_types {
        let start_date = state.get_start_date(&subject_type, default_start);

        if let Some(end) = window_end {
            if start_date >= end {
                continue;
            }
        }

        let mut dr_builder = DateRangeBuilder::new()
            .date_type(DateType::PermanentStorage)
            .from(start_date.to_rfc3339())
            .restrict_to_permanent_storage_hwm_date(true);

        if let Some(end_dt) = window_end.as_ref() {
            dr_builder = dr_builder.to(end_dt.to_rfc3339());
        }

        let date_range = dr_builder.build().map_err(|e| {
            KsefError::ApplicationError(0, format!("Failed to build DateRange: {}", e))
        })?;

        let query = QueryCriteriaBuilder::new()
            .subject_type(subject_type.clone())
            .date_range(date_range)
            .build()
            .map_err(|e| {
                KsefError::ApplicationError(0, format!("Failed to build QueryCriteria: {}", e))
            })?;

        let export_result = client.export_invoices(query).await?;

        let mut invoices_in_batch = HashMap::new();
        let mut metadata_in_batch = HashMap::new();

        for part in &export_result.parts {
            let cursor = std::io::Cursor::new(&part.content);
            let mut archive = zip::ZipArchive::new(cursor).map_err(|e| {
                KsefError::ApplicationError(0, format!("Failed to open zip archive: {}", e))
            })?;

            for i in 0..archive.len() {
                let mut file = archive.by_index(i).map_err(|e| {
                    KsefError::ApplicationError(0, format!("Failed to read file in zip: {}", e))
                })?;

                if file.name().ends_with(".xml") {
                    let mut xml_content = String::new();
                    file.read_to_string(&mut xml_content).map_err(|e| {
                        KsefError::ApplicationError(0, format!("Failed to read xml content: {}", e))
                    })?;
                    invoices_in_batch.insert(file.name().to_string(), xml_content);
                } else if file.name().ends_with(".json") && file.name().contains("metadata") {
                    let mut json_content = String::new();
                    file.read_to_string(&mut json_content).map_err(|e| {
                        KsefError::ApplicationError(
                            0,
                            format!("Failed to read metadata json: {}", e),
                        )
                    })?;

                    let metadata_pkg: InvoicePackageMetadata = serde_json::from_str(&json_content)
                        .map_err(|e| {
                            KsefError::ApplicationError(
                                0,
                                format!("Failed to parse metadata json: {}", e),
                            )
                        })?;

                    for meta in metadata_pkg.invoices {
                        metadata_in_batch.insert(meta.ksef_number.clone(), meta);
                    }
                }
            }
        }

        for (ksef_number, meta) in metadata_in_batch {
            if processed_ksef_numbers.contains(&ksef_number) {
                continue;
            }

            let content = invoices_in_batch
                .iter()
                .find(|(name, _)| name.contains(&ksef_number));

            if let Some((_, xml)) = content {
                processed_ksef_numbers.insert(ksef_number.clone());
                all_fetched_invoices.push(FetchedInvoice {
                    metadata: meta,
                    content: xml.clone(),
                });
            } else {
                println!(
                    "Warning: Content for KSeF number {} not found in zip parts",
                    ksef_number
                );
            }
        }

        state.update_continuation_point(&subject_type, &export_result);
    }

    Ok(all_fetched_invoices)
}
