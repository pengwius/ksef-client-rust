use crate::client::error::KsefError;

pub mod error;

mod session;

pub struct KsefClient;

impl KsefClient {
    pub fn new() -> Self {
        Self
    }

    pub fn add(&self, left: u8, right: u8) -> Result<u8, KsefError> {
        session::add(self, left, right)
    }
}
