use crate::common;
use ksef_client::prelude::*;
use serde_json::json;

#[tokio::test]
async fn test_get_certificates_limits() {
    let client: ksef_client::KsefClient = common::authorize_client().await;

    println!("Getting certificate limits...");
    match client.get_certificates_limits().await {
        Ok(mut limits) => {
            println!("Certificate limits retrieved successfully.");
            println!("Can request: {}", limits.can_request);
            println!(
                "Enrollment limit: {}/{}",
                limits.enrollment.remaining, limits.enrollment.limit
            );
            println!(
                "Certificate limit: {}/{}",
                limits.certificate.remaining, limits.certificate.limit
            );

            if limits.certificate.remaining == 0 {
                println!(
                    "Certificate remaining quota is 0 — attempting to increase test limits..."
                );

                let increase_by: i32 = 100;
                let new_certificate_limit = limits.certificate.limit + increase_by;
                let new_enrollment_limit = limits.enrollment.limit + increase_by;

                let body = json!({
                    "SubjectIdentifierType": "Nip",
                    "Certificate": { "MaxCertificates": new_certificate_limit },
                    "Enrollment": { "MaxEnrollments": new_enrollment_limit }
                });

                let testdata_path = "/v2/testdata/limits/subject/certificate";
                let url = client.url_for(testdata_path);

                let resp = client
                    .client
                    .post(&url)
                    .bearer_auth(&client.access_token.access_token)
                    .json(&body)
                    .send()
                    .await;

                match resp {
                    Ok(r) => {
                        if r.status().is_success() {
                            println!(
                                "Successfully requested increased certificate/enrollment limits via test API."
                            );

                            match client.get_certificates_limits().await {
                                Ok(new_limits) => {
                                    println!("New limits: {:#?}", new_limits);
                                    limits = new_limits;
                                }
                                Err(e) => {
                                    panic!(
                                        "Failed to re-fetch certificate limits after increasing test limits: {:?}",
                                        e
                                    );
                                }
                            }
                        } else {
                            panic!(
                                "Test API returned non-success status when trying to increase limits: {}",
                                r.status()
                            );
                        }
                    }
                    Err(e) => {
                        panic!(
                            "Failed to call test API to increase certificate limits: {:?}",
                            e
                        );
                    }
                }
            }

            println!(
                "Final Enrollment limit: {}/{}",
                limits.enrollment.remaining, limits.enrollment.limit
            );
            println!(
                "Final Certificate limit: {}/{}",
                limits.certificate.remaining, limits.certificate.limit
            );

            assert!(limits.enrollment.limit >= 0);
            assert!(limits.certificate.limit >= 0);
        }
        Err(e) => panic!("Failed to get certificate limits: {:?}", e),
    }
}
