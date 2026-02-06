use super::KsefClient;
use super::error::KsefError;

pub fn add(_client: &KsefClient, left: u8, right: u8) -> Result<u8, KsefError> {
    left.checked_add(right)
        .ok_or_else(|| KsefError::AddError(format!("overflow when adding {} and {}", left, right)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let client = KsefClient::new();
        let result = add(&client, 2, 2);
        assert_eq!(result, Ok(4));
    }

    #[test]
    fn add_overflow_test() {
        let client = KsefClient::new();
        let result = add(&client, u8::MAX, 1);
        assert_eq!(result, Err(KsefError::AddError(format!("overflow when adding {} and {}", u8::MAX, 1))));
    }
}
