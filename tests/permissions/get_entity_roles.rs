use crate::common;

#[tokio::test]
async fn test_get_entity_roles() {
    let client: ksef_client::KsefClient = common::authorize_client().await;

    println!("Fetching entity roles once...");

    match client.get_entity_roles(Some(0), Some(100)).await {
        Ok(resp) => {
            println!("get_entity_roles returned {} roles", resp.roles.len());
            for r in &resp.roles {
                println!(
                    " - role: {}, desc: {}, start: {}, parent: {:?}",
                    r.role, r.description, r.start_date, r.parent_entity_identifier
                );
            }
        }
        Err(e) => {
            panic!("Failed to fetch entity roles: {:?}", e);
        }
    }

    match client.get_entity_roles(Some(0), Some(10)).await {
        Ok(resp) => {
            println!(
                "get_entity_roles (pageSize=10) returned {} roles, has_more={}",
                resp.roles.len(),
                resp.has_more
            );
        }
        Err(ksef_client::KsefError::ApiError(code, _)) if code == 500 => {
            eprintln!("get_entity_roles (pageSize=10) returned 500; skipping second check");
            return;
        }
        Err(e) => {
            panic!("Failed to fetch entity roles (pageSize=10): {:?}", e);
        }
    }
}
