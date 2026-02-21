use crate::common;

#[test]
fn test_get_enrollment_data() {
    let client = common::authorize_client();

    println!("Getting enrollment data...");
    match client.get_enrollment_data() {
        Ok(data) => {
            println!("Enrollment data retrieved successfully.");
            println!("Common Name: {}", data.common_name);
            println!("Country Name: {}", data.country_name);

            if let Some(gn) = &data.given_name {
                println!("Given Name: {}", gn);
            }
            if let Some(sn) = &data.surname {
                println!("Surname: {}", sn);
            }
            if let Some(serial) = &data.serial_number {
                println!("Serial Number: {}", serial);
            }
            if let Some(uid) = &data.unique_identifier {
                println!("Unique Identifier: {}", uid);
            }
            if let Some(org) = &data.organization_name {
                println!("Organization Name: {}", org);
            }
            if let Some(org_id) = &data.organization_identifier {
                println!("Organization Identifier: {}", org_id);
            }

            assert!(
                !data.common_name.is_empty(),
                "Common name should not be empty"
            );
            assert!(
                !data.country_name.is_empty(),
                "Country name should not be empty"
            );
        }
        Err(e) => panic!("Failed to get enrollment data: {:?}", e),
    }
}
