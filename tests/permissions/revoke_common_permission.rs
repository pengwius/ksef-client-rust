use crate::common;
use ksef_client::{
    GrantPersonPermissionsRequest, GrantSubjectIdentifierType, PersonById, PersonPermissionType,
    SubjectDetails, SubjectDetailsType, SubjectIdentifier,
};
use serde_json::Value;
use tokio::time::{Duration, sleep};

#[tokio::test]
async fn test_grant_and_revoke_common_permission_workflow() {
    let client: ksef_client::KsefClient = common::authorize_client().await;
    let target_nip: String = common::generate_random_nip().await;

    let request = GrantPersonPermissionsRequest::builder()
        .with_subject_identifier(SubjectIdentifier {
            identifier_type: GrantSubjectIdentifierType::Nip,
            value: target_nip.clone(),
        })
        .with_permissions(vec![
            PersonPermissionType::InvoiceRead,
            PersonPermissionType::InvoiceWrite,
        ])
        .with_description("Integration test: grant and revoke common permission")
        .with_subject_details(SubjectDetails {
            subject_details_type: SubjectDetailsType::PersonByIdentifier,
            person_by_id: Some(PersonById {
                first_name: "Test".to_string(),
                last_name: "User".to_string(),
            }),
            person_by_fp_with_id: None,
            person_by_fp_no_id: None,
        })
        .build()
        .expect("Failed to build grant request");

    let grant_resp = client
        .grant_person_permissions(request)
        .await
        .expect("Failed to grant person permissions");
    assert!(
        !grant_resp.reference_number.is_empty(),
        "Grant response reference number should not be empty"
    );
    println!(
        "Granted permissions. Reference number: {}",
        grant_resp.reference_number
    );

    let mut permission_id: Option<String> = None;
    for _ in 0..10 {
        let list: Value = client
            .get_common_permissions()
            .await
            .expect("Failed to fetch common permissions");

        let candidates: Vec<Value> = if list.is_array() {
            list.as_array().unwrap().clone()
        } else if let Some(arr) = list.get("items").and_then(|v| v.as_array()) {
            arr.clone()
        } else if let Some(arr) = list.get("grants").and_then(|v| v.as_array()) {
            arr.clone()
        } else if let Some(arr) = list.get("data").and_then(|v| v.as_array()) {
            arr.clone()
        } else {
            list.as_object()
                .map(|obj| obj.values().cloned().collect())
                .unwrap_or_default()
        };

        for item in candidates {
            let id_opt = item
                .get("id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .or_else(|| {
                    item.get("permissionId")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                })
                .or_else(|| {
                    item.get("permission_id")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                });

            let subject_value_opt = item
                .get("subjectIdentifier")
                .and_then(|si| si.get("value"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .or_else(|| {
                    item.get("subject").and_then(|s| {
                        s.get("value")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string())
                    })
                })
                .or_else(|| {
                    item.get("value")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                });

            if let (Some(id), Some(subject_value)) = (id_opt, subject_value_opt) {
                if subject_value == target_nip {
                    permission_id = Some(id);
                    break;
                }
            }
        }

        if permission_id.is_some() {
            break;
        }

        sleep(Duration::from_millis(500)).await;
    }

    let permission_id = match permission_id {
        Some(id) => id,
        None => {
            panic!(
                "Failed to find permission id for subject {} in common grants",
                target_nip
            );
        }
    };

    println!("Found permission id: {}", permission_id);

    let revoke_resp = client
        .revoke_common_permission(&permission_id)
        .await
        .expect("Failed to revoke common permission");
    assert!(
        !revoke_resp.reference_number.is_empty(),
        "Revoke response reference number should not be empty"
    );
    println!(
        "Revoked permission. Reference number: {}",
        revoke_resp.reference_number
    );

    let mut still_present = true;
    for _ in 0..10 {
        let list: Value = client
            .get_common_permissions()
            .await
            .expect("Failed to fetch common permissions after revoke");

        let mut found = false;
        let candidates: Vec<Value> = if list.is_array() {
            list.as_array().unwrap().clone()
        } else if let Some(arr) = list.get("items").and_then(|v| v.as_array()) {
            arr.clone()
        } else if let Some(arr) = list.get("grants").and_then(|v| v.as_array()) {
            arr.clone()
        } else if let Some(arr) = list.get("data").and_then(|v| v.as_array()) {
            arr.clone()
        } else {
            list.as_object()
                .map(|obj| obj.values().cloned().collect())
                .unwrap_or_default()
        };

        for item in candidates {
            let id_opt = item
                .get("id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .or_else(|| {
                    item.get("permissionId")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                })
                .or_else(|| {
                    item.get("permission_id")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                });

            if let Some(id) = id_opt {
                if id == permission_id {
                    found = true;
                    break;
                }
            }
        }

        if !found {
            still_present = false;
            break;
        }

        sleep(Duration::from_millis(500)).await;
    }

    assert!(
        !still_present,
        "Permission id {} still present after revoke",
        permission_id
    );
}
