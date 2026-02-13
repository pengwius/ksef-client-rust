use chrono::Utc;
use clap::Parser;
use ksef_client::{ContextIdentifierType, KsefClient, SubjectIdentifierType};
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

fn generate_random_nip() -> String {
    use rand::{Rng, distributions::Uniform};
    let mut rng = rand::thread_rng();
    let digits = Uniform::from(0..10);
    (0..10).map(|_| rng.sample(&digits).to_string()).collect()
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Generates a self-signed test certificate")]
struct Args {
    #[arg(long, default_value = "screen")]
    output: String,

    #[arg(long, default_value = "1234567890")]
    nip: String,

    #[arg(long = "given-name", default_value = "Eugeniusz")]
    given_name: String,

    #[arg(long = "surname", default_value = "Fakturowski")]
    surname: String,

    #[arg(long = "serial-prefix", default_value = "TINPL")]
    serial_prefix: String,

    #[arg(long = "common-name", default_value = "E F")]
    common_name: String,

    #[arg(long, default_value = "EUGE Sp. z o.o.")]
    organization: String,

    #[arg(long = "out-dir", default_value = ".")]
    out_dir: String,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let output_mode = if args.output.to_lowercase() == "file" {
        "file"
    } else {
        "screen"
    };

    let nip = if args.nip.trim().is_empty() {
        let rnd = generate_random_nip();
        println!("[1] Preparing NIP...");
        println!("    NIP: {} (random)", rnd);
        rnd
    } else {
        println!("[1] Preparing NIP...");
        println!("    NIP: {} (from argument)", args.nip.trim());
        args.nip.trim().to_string()
    };

    println!("[1.1] Certificate subject params:");
    println!("    GivenName: {}", &args.given_name);
    println!("    Surname: {}", &args.surname);
    println!("    Serial prefix: {}", &args.serial_prefix);
    println!("    Organization: {}", &args.organization);
    println!("    CommonName: {}", &args.common_name);

    println!("[3] Getting AuthTokenRequest...");
    let mut client = KsefClient::new();

    let auth_token_request = match client.get_auth_token_request(
        &nip,
        ContextIdentifierType::Nip,
        SubjectIdentifierType::CertificateSubject,
    ) {
        Ok(req) => req,
        Err(e) => {
            eprintln!("Unable to get AuthTokenRequest: {}", e);
            return ExitCode::FAILURE;
        }
    };

    println!("[4] Request serialization to XML (unsigned)...");
    let unsigned_xml = auth_token_request.to_xml();
    println!(
        "-----Unsigned XML-----\n{}\n-----END: Unsigned XML-----",
        unsigned_xml
    );

    println!("[5] Generating self-signed test certificate...");
    match client.xades.gen_selfsign_cert(
        &args.given_name,
        &args.surname,
        &args.serial_prefix,
        &nip,
        &args.common_name,
    ) {
        Ok(()) => {
            println!("    Self-signed certificate has been generated and loaded into XadesSigner.");
        }
        Err(e) => {
            eprintln!("Unable to generate self-signed certificate: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let timestamp = Utc::now().format("%Y%m%d-%H%M%S").to_string();

    println!("[6] Signing XML (XAdES)...");
    let signed_xml = match client.xades.sign(&unsigned_xml) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Unable to sign XML: {}", e);
            return ExitCode::FAILURE;
        }
    };

    if output_mode == "file" {
        let out_dir = PathBuf::from(&args.out_dir);
        let signed_file = out_dir.join(format!("signed-auth-{}.xml", timestamp));
        if let Err(e) = fs::write(&signed_file, signed_xml.as_bytes()) {
            eprintln!("Unable to write signed XML to file: {}", e);
            return ExitCode::FAILURE;
        }
        println!("Saved signed XML to file: {}", signed_file.display());
    } else {
        println!(
            "-----Signed XML-----\n{}\n-----END: Signed XML-----",
            signed_xml
        );
    }

    println!("[7] Sending signed XML to KSeF...");
    match client.authenticate_by_xades_signature(signed_xml) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Unable to submit signed XML for authentication: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let auth_tokens = client.auth_token();

    println!(
        "    AuthenticationToken: {}",
        auth_tokens.authentication_token
    );
    println!("    ReferenceNumber: {}", auth_tokens.reference_number);

    println!("[8] Requesting authentication status (polling)...");
    let is_authenticated: bool = match client.get_auth_status() {
        Ok(status) => status,
        Err(e) => {
            eprintln!("Unable to get authentication status: {}", e);
            return ExitCode::FAILURE;
        }
    };

    if is_authenticated {
        println!("    Status: Authentication completed successfully.");
    } else {
        println!("    Status: Authentication still in progress or failed.");
        return ExitCode::FAILURE;
    }

    println!("[9] Getting access token...");
    match client.get_access_token() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Unable to get access token: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let access_tokens = client.access_token();

    println!("    AccessToken: {}", access_tokens.access_token);
    println!("    RefreshToken: {}", access_tokens.refresh_token);

    println!("Finished successfully.");
    ExitCode::SUCCESS
}
