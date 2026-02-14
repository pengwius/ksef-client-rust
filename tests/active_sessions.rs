mod common;

#[test]
fn test_get_active_sessions() {
    let client = common::authorize_client();

    match client.get_active_sessions(None) {
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
