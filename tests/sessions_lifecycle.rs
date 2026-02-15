use std::time::Duration;

mod common;

use ksef_client::{ContextIdentifierType, KsefClient, SubjectIdentifierType};

fn authorize_client_for_nip(nip: &str) -> KsefClient {
    let mut client = KsefClient::new();
    let given_name = "Test";
    let surname = "User";
    let serial_prefix = "TINPL";
    let common_name = "Test User";

    let auth_token_request = client
        .get_auth_token_request(
            nip,
            ContextIdentifierType::Nip,
            SubjectIdentifierType::CertificateSubject,
        )
        .expect("Failed to get auth token request");

    let unsigned_xml = auth_token_request.to_xml();

    client
        .xades
        .gen_selfsign_cert(given_name, surname, serial_prefix, nip, common_name)
        .expect("Failed to generate self-signed certificate");

    let signed_xml = client
        .xades
        .sign(&unsigned_xml)
        .expect("Failed to sign XML");

    match client.authenticate_by_xades_signature(signed_xml) {
        Ok(_) => {}
        Err(e) => {
            panic!("Authentication request submission failed: {:?}", e);
        }
    }

    match client.get_auth_status() {
        Ok(true) => {}
        Ok(false) => {
            panic!("Authentication not successful");
        }
        Err(e) => {
            panic!("Error checking auth status: {:?}", e);
        }
    }

    std::thread::sleep(Duration::from_millis(300));

    client
}

#[test]
fn test_revoke_session_by_reference() {
    let nip = common::generate_random_nip();

    let client_a = authorize_client_for_nip(&nip);
    let client_b = authorize_client_for_nip(&nip);

    let ref_a = client_a.auth_token().reference_number.clone();
    let ref_b = client_b.auth_token().reference_number.clone();

    println!("Client A ref: {}", ref_a);
    println!("Client B ref: {}", ref_b);

    assert_ne!(
        ref_a, ref_b,
        "Expected two distinct session reference numbers"
    );

    match client_a.revoke_session(&ref_b) {
        Ok(()) => {
            println!("Client A revoked Client B's session (reference {})", ref_b);
        }
        Err(e) => {
            panic!("Failed to revoke session by reference: {:?}", e);
        }
    }

    std::thread::sleep(Duration::from_millis(300));

    let resp = client_a
        .get_active_sessions(None)
        .expect("Failed to query active sessions");
    let sessions = resp.items;

    let found = sessions.iter().find(|s| s.reference_number == ref_b);

    if let Some(s) = found {
        println!("Found session {} with status code {}", ref_b, s.status.code);
        assert!(
            s.status.code == 425 || s.status.code >= 400,
            "Expected revoked or non-active status for the revoked session"
        );
    } else {
        println!(
            "Session {} not present in session list after revoke (acceptable)",
            ref_b
        );
    }
}

#[test]
fn test_two_session_lifecycle_revoke_other_and_self() {
    let nip = common::generate_random_nip();

    let client1 = authorize_client_for_nip(&nip);
    let client2 = authorize_client_for_nip(&nip);

    let ref1 = client1.auth_token().reference_number.clone();
    let ref2 = client2.auth_token().reference_number.clone();

    println!("Lifecycle test: ref1={}, ref2={}", ref1, ref2);

    assert_ne!(
        ref1, ref2,
        "Expected two distinct session reference numbers"
    );

    client1
        .revoke_session(&ref2)
        .expect("client1 failed to revoke client2 by reference");

    std::thread::sleep(Duration::from_millis(300));

    let resp_after_revoke = client1
        .get_active_sessions(None)
        .expect("Failed to query sessions after revoke");
    let sessions_after = resp_after_revoke.items;
    let sess2 = sessions_after.iter().find(|s| s.reference_number == ref2);

    if let Some(s) = sess2 {
        println!(
            "After external revoke: session {} status {}",
            ref2, s.status.code
        );
        assert!(
            s.status.code == 425 || s.status.code >= 400,
            "Expected client2 session to be revoked after client1 revoked it"
        );
    } else {
        println!(
            "After external revoke: session {} not present (treated as revoked)",
            ref2
        );
    }

    match client2.revoke_current_session() {
        Ok(()) => {
            println!("client2 revoke_current_session returned Ok");
        }
        Err(e) => {
            println!(
                "client2 revoke_current_session returned Err (acceptable if already revoked): {:?}",
                e
            );
        }
    }

    std::thread::sleep(Duration::from_millis(300));
    let final_resp = client1
        .get_active_sessions(None)
        .expect("Failed to query sessions final");
    let final_sessions = final_resp.items;
    let final_sess2 = final_sessions.iter().find(|s| s.reference_number == ref2);

    if let Some(s) = final_sess2 {
        println!("Final check: session {} status {}", ref2, s.status.code);
        assert!(
            s.status.code == 425 || s.status.code >= 400,
            "Expected client2 session to be revoked in final check"
        );
    } else {
        println!("Final check: session {} not present (revoked)", ref2);
    }

    match client1.revoke_current_session() {
        Ok(()) => {
            println!("client1 revoked its own session successfully");
        }
        Err(e) => {
            panic!("client1 failed to revoke its own session: {:?}", e);
        }
    }

    std::thread::sleep(Duration::from_millis(300));
    let end_resp = client2
        .get_active_sessions(None)
        .expect("Failed to query sessions after client1 self-revoke");
    let end_sessions = end_resp.items;
    let end_sess1 = end_sessions.iter().find(|s| s.reference_number == ref1);

    if let Some(s) = end_sess1 {
        println!("End check: session {} status {}", ref1, s.status.code);
        assert!(
            s.status.code == 425 || s.status.code >= 400,
            "Expected client1 session to be revoked in end check"
        );
    } else {
        println!("End check: session {} not present (revoked)", ref1);
    }
}
