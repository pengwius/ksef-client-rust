use ksef_client::prelude::*;
mod common;
use secrecy::ExposeSecret;

#[tokio::test]
async fn test_access_token_retrieval() {
    let client: ksef_client::KsefClient = common::authorize_client().await;

    let tokens = client.access_token();

    assert!(
        !tokens.access_token.expose_secret().is_empty(),
        "Access token should not be empty after authorization"
    );
    assert!(
        !tokens.refresh_token.expose_secret().is_empty(),
        "Refresh token should not be empty after authorization"
    );
}

#[tokio::test]
async fn test_access_token_refresh() {
    let mut client: ksef_client::KsefClient = common::authorize_client().await;

    let initial_access_token = client.access_token().access_token.clone();
    assert!(
        !initial_access_token.expose_secret().is_empty(),
        "Initial access token required for refresh test"
    );

    println!("Refreshing access token...");
    match client.refresh_access_token().await {
        Ok(_) => {
            let new_tokens = client.access_token();
            assert!(
                !new_tokens.access_token.expose_secret().is_empty(),
                "New access token should not be empty"
            );
            println!(
                "Token refreshed. Old: {}..., New: {}...",
                &initial_access_token.expose_secret()
                    [..10.min(initial_access_token.expose_secret().len())],
                &new_tokens.access_token.expose_secret()
                    [..10.min(new_tokens.access_token.expose_secret().len())]
            );
        }
        Err(e) => {
            panic!("Failed to refresh access token: {:?}", e);
        }
    }
}
