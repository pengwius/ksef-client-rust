use crate::common;

#[test]
fn test_get_certificates_limits() {
    let client = common::authorize_client();

    println!("Getting certificate limits...");
    match client.get_certificates_limits() {
        Ok(limits) => {
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

            assert!(limits.enrollment.limit >= 0);
            assert!(limits.certificate.limit >= 0);
        }
        Err(e) => panic!("Failed to get certificate limits: {:?}", e),
    }
}
