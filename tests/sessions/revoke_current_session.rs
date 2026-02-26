use crate::common;

#[tokio::test]
async fn test_revoke_current_session() {
    let client: ksef_client::KsefClient = common::authorize_client().await;

    match client.revoke_current_session().await {
        Ok(()) => {
            println!("Successfully revoked current session");
        }
        Err(e) => {
            panic!("Failed to revoke current session: {:?}", e);
        }
    }
}
