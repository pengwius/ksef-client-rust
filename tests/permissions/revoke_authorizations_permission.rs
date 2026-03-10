use crate::common;
use ksef_client::permissions::{
    AuthorizationAuthorizedIdentifier, AuthorizationPermissionType, AuthorizationSubjectDetails,
    AuthorizationSubjectIdentifier, AuthorizationSubjectIdentifierType,
    GetAuthorizationsPermissionsRequest, GrantAuthorizationPermissionsRequest, QueryType,
};
use ksef_client::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_revoke_authorizations_permission() {
    let client: ksef_client::KsefClient = common::authorize_client().await;
    let target_nip: String = common::generate_random_nip().await;

    println!("Granting authorization to NIP: {}", target_nip);

    let grant_request = GrantAuthorizationPermissionsRequest::builder()
        .with_subject_identifier(AuthorizationSubjectIdentifier {
            identifier_type: AuthorizationSubjectIdentifierType::Nip,
            value: target_nip.clone(),
        })
        .with_permission(AuthorizationPermissionType::SelfInvoicing)
        .with_description("Test authorization grant for revocation")
        .with_subject_details(AuthorizationSubjectDetails {
            full_name: "Test Entity".to_string(),
        })
        .build()
        .expect("Failed to build request");

    match client.grant_authorization_permissions(grant_request).await {
        Ok(resp) => {
            println!("Grant response: {:#?}", resp);
            if let Some(code) = resp.status_code() {
                assert_eq!(code, 200, "Expected grant status code 200");
            }
        }
        Err(e) => {
            panic!("Failed to grant authorization permissions: {:?}", e);
        }
    }

    println!("Waiting for permission propagation...");
    sleep(Duration::from_secs(2)).await;

    let mut permission_id = String::new();
    let mut found = false;

    let query_request = GetAuthorizationsPermissionsRequest {
        authorizing_identifier: None,
        authorized_identifier: Some(AuthorizationAuthorizedIdentifier {
            identifier_type: "Nip".to_string(),
            value: target_nip.clone(),
        }),
        query_type: QueryType::Granted,
        permission_types: Some(vec!["SelfInvoicing".to_string()]),
    };

    for i in 0..10 {
        println!("Attempt {} to find permission...", i + 1);

        match client
            .get_authorizations_permissions(Some(0), Some(100), query_request.clone())
            .await
        {
            Ok(resp) => {
                println!(
                    "Get permissions found {} entries",
                    resp.authorization_grants.len()
                );
                if let Some(perm) = resp
                    .authorization_grants
                    .iter()
                    .find(|p| p.authorized_entity_identifier.value == target_nip)
                {
                    println!("Found permission ID: {}", perm.id);
                    permission_id = perm.id.clone();
                    found = true;
                    break;
                }
            }
            Err(e) => {
                println!("Failed to get permissions: {:?}", e);
            }
        }
        sleep(Duration::from_secs(2)).await;
    }

    assert!(found, "Could not find the granted permission after retries");

    println!("Revoking permission ID: {}", permission_id);
    match client
        .revoke_authorizations_permission(&permission_id)
        .await
    {
        Ok(resp) => {
            println!("Revoke response: {:#?}", resp);
            if let Some(code) = resp.status_code() {
                assert_eq!(code, 200, "Expected revoke status code 200");
            }
        }
        Err(e) => {
            panic!("Failed to revoke permission: {:?}", e);
        }
    }
}
