mod common;

#[test]
fn test_access_token_retrieval() {
    let client = common::authorize_client();

    let tokens = client.access_token();

    assert!(
        !tokens.access_token.is_empty(),
        "Access token should not be empty after authorization"
    );
    assert!(
        !tokens.refresh_token.is_empty(),
        "Refresh token should not be empty after authorization"
    );
}

#[test]
fn test_access_token_refresh() {
    let mut client = common::authorize_client();

    let initial_access_token = client.access_token().access_token.clone();
    assert!(
        !initial_access_token.is_empty(),
        "Initial access token required for refresh test"
    );

    println!("Refreshing access token...");
    match client.refresh_access_token() {
        Ok(_) => {
            let new_tokens = client.access_token();
            assert!(
                !new_tokens.access_token.is_empty(),
                "New access token should not be empty"
            );
            println!(
                "Token refreshed. Old: {}..., New: {}...",
                &initial_access_token[..10.min(initial_access_token.len())],
                &new_tokens.access_token[..10.min(new_tokens.access_token.len())]
            );
        }
        Err(e) => {
            panic!("Failed to refresh access token: {:?}", e);
        }
    }
}
