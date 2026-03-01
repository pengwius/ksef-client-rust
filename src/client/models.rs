use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Test,
    Prod,
}

impl Environment {
    pub fn base_url(&self) -> &'static str {
        match self {
            Environment::Test => "https://api-test.ksef.mf.gov.pl/api",
            Environment::Prod => "https://api.ksef.mf.gov.pl/api",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextIdentifier {
    #[serde(rename = "type")]
    pub id_type: ContextIdentifierType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ContextIdentifierType {
    Nip,
    InternalId,
    NipVatUe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Encryption {
    #[serde(rename = "encryptedSymmetricKey")]
    pub encrypted_symmetric_key: String,
    #[serde(rename = "initializationVector")]
    pub initialization_vector: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormCode {
    #[serde(rename = "systemCode")]
    pub system_code: String,
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoicePayload {
    pub filename: String,
    pub content: Vec<u8>,
}
