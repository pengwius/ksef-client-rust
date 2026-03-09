use ksef_client::*;
use crate::common;
use ksef_client::PersonsPermissionsRequest;

#[tokio::test]
async fn test_get_persons_permissions() {
    let client: ksef_client::KsefClient = common::authorize_client().await;

    let builder =
        PersonsPermissionsRequest::builder().with_query_type("PermissionsInCurrentContext");
    let request = builder.build().expect("Failed to build request");

    let result = client
        .get_persons_permissions(Some(0), Some(10), Some(request))
        .await;

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
        assert!(
            !p.authorized_identifier.value.is_empty(),
            "authorized_identifier value should not be empty"
        );
        assert!(
            !p.permission_scope.is_empty(),
            "permission_scope should not be empty"
        );
    }
}
