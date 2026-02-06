use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum KsefError {
    #[error("Adding failed: {0}")]
    AddError(String),
}
