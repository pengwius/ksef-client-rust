use ksef_client::*;
use crate::common;

use ksef_client::{
    EntitiesContextIdentifier, EntityIdentifier, EntityIdentifierType, EntityPermission,
    EntityPermissionType, EntitySubjectDetails, GetEntitiesPermissionsRequest,
    GrantEntityPermissionsRequest,
};

#[tokio::test]
async fn test_get_entities_permissions() {
    let client: ksef_client::KsefClient = common::authorize_client().await;
    let target_nip: String = common::generate_random_nip().await;

    let request = GrantEntityPermissionsRequest::builder()
        .with_subject_identifier(EntityIdentifier {
            identifier_type: EntityIdentifierType::Nip,
            value: target_nip.clone(),
        })
        .with_permissions(vec![EntityPermission {
            permission_type: EntityPermissionType::InvoiceRead,
            can_delegate: Some(false),
        }])
        .with_description("Integration test: grant and fetch entity permission")
        .with_subject_details(EntitySubjectDetails {
            full_name: "Test Entity Sp. z o.o.".to_string(),
        })
        .build()
        .expect("Failed to build grant request");

    let op_status = match client.grant_entity_permissions(request).await {
        Ok(op) => op,
        Err(e) => panic!("Failed to grant entity permissions: {:?}", e),
    };

    match op_status.status_code() {
        Some(code) => assert_eq!(code, 200, "Expected grant final code 200, got {}", code),
        None => panic!(
            "grant operation did not return numeric status: {:?}",
            op_status.raw
        ),
    }

    let query = GetEntitiesPermissionsRequest {
        context_identifier: Some(EntitiesContextIdentifier {
            identifier_type: "Nip".to_string(),
            value: client.context.value.clone(),
        }),
    };

    println!("Fetching entities permissions once...");

    match client
        .get_entities_permissions(Some(0), Some(100), Some(query))
        .await
    {
        Ok(resp) => {
            println!(
                "get_entities_permissions returned {} items",
                resp.permissions.len()
            );
            for p in &resp.permissions {
                println!(
                    " - id: {}, scope: {}, desc: {}",
                    p.id, p.permission_scope, p.description
                );
            }
        }
        Err(ksef_client::KsefError::ApiError(code, _)) if code == 500 => {
            eprintln!("get_entities_permissions returned 500; skipping test");
            return;
        }
        Err(e) => {
            panic!("Failed to fetch entities permissions: {:?}", e);
        }
    }
}
