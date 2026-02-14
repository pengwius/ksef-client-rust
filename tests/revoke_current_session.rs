mod common;

#[test]
fn test_revoke_current_session() {
    let client = common::authorize_client();

    match client.revoke_current_session() {
        Ok(()) => {
            println!("Successfully revoked current session");
        }
        Err(e) => {
            panic!("Failed to revoke current session: {:?}", e);
        }
    }
}
