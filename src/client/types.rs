use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;

macro_rules! define_string_newtype {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
        pub struct $name(String);

        impl $name {
            pub fn new(s: impl Into<String>) -> Self {
                Self(s.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }

            pub fn into_inner(self) -> String {
                self.0
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self(s)
            }
        }

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self(s.to_string())
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

define_string_newtype!(Nip, "Tax Identification Number (NIP).");
define_string_newtype!(KsefNumber, "KSeF Invoice Number.");
define_string_newtype!(
    ReferenceNumber,
    "Reference Number (e.g. for a session or an operation)."
);
define_string_newtype!(CertificateSerialNumber, "Certificate Serial Number.");
define_string_newtype!(InvoiceHash, "Invoice Hash.");
