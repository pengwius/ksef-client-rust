use crate::common;

use ksef_client::GetEuEntitiesPermissionsRequest;
use ksef_client::KsefError;

#[tokio::test]
async fn test_get_eu_entities_permissions() {
    let client: ksef_client::KsefClient = common::authorize_client().await;

    println!("Fetching EU entities permissions once...");

    match client
        .get_eu_entities_permissions(Some(0), Some(100), None)
        .await
    {
        Ok(resp) => {
            println!(
                "get_eu_entities_permissions returned {} items",
                resp.permissions.len()
            );
            for p in &resp.permissions {
                println!(
                    " - id: {}, vatUe: {}, scope: {}, desc: {}",
                    p.id, p.vat_ue_identifier, p.permission_scope, p.description
                );
            }
        }
        Err(KsefError::ApiError(code, _)) if code == 500 => {
            eprintln!("get_eu_entities_permissions returned 500; skipping test");
            return;
        }
        Err(e) => {
            panic!("Failed to fetch eu entities permissions: {:?}", e);
        }
    }

    let req = GetEuEntitiesPermissionsRequest {
        vat_ue_identifier: None,
        authorized_fingerprint_identifier: None,
        permission_types: None,
    };

    match client
        .get_eu_entities_permissions(Some(0), Some(50), Some(req))
        .await
    {
        Ok(resp) => {
            println!(
                "get_eu_entities_permissions (with empty typed body) returned {} items",
                resp.permissions.len()
            );
        }
        Err(KsefError::ApiError(code, _)) if code == 500 => {
            eprintln!("get_eu_entities_permissions (typed) returned 500; skipping second check");
            return;
        }
        Err(e) => {
            panic!("Failed to fetch eu entities permissions (typed): {:?}", e);
        }
    }
}
