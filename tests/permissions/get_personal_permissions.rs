use ksef_client::prelude::*;
use crate::common;

#[tokio::test]
async fn test_get_personal_permissions() {
    let client: ksef_client::KsefClient = common::authorize_client().await;

    let result = client.get_personal_permissions(Some(0), Some(10), None).await;
    assert!(
        result.is_ok(),
        "Expected Ok result, got Err: {:?}",
        result.err()
    );

    let resp = result.unwrap();
    assert!(
        resp.has_more == true || resp.has_more == false,
        "has_more should be a boolean"
    );

    if !resp.permissions.is_empty() {
        let p = &resp.permissions[0];
        assert!(!p.id.is_empty(), "Permission id should not be empty");
        assert!(!p.permission_scope.is_empty(), "permission_scope should not be empty");
    }
}
