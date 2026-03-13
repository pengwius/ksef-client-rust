use ksef_client::prelude::*;
use crate::common;

#[tokio::test]
async fn test_get_active_sessions() {
    let client: ksef_client::KsefClient = common::authorize_client().await;

    match client.get_active_sessions(None).await {
        Ok(resp) => {
            let sessions = resp.items;
            println!("Retrieved {} active sessions", sessions.len());

            assert!(
                sessions.iter().all(|s| !s.reference_number.is_empty()),
                "Every session should have a non-empty reference number"
            );

            if let Some(first) = sessions.first() {
                let _ = first.is_current;

                println!(
                    "First session: reference={}, method={:?}, start_date={}",
                    first.reference_number, first.authentication_method, first.start_date
                );
            }
        }
        Err(e) => {
            panic!("Failed to retrieve active sessions: {:?}", e);
        }
    }
}
