use rand::{distributions::Uniform, Rng};
use clap::Parser;
use std::process::ExitCode;
use ksef_client::KsefClient;

#[derive(Parser, Debug)]
#[command(author, version, about = "Generuje samopodpisany certyfikat testowy (PFX + CER)")]
struct Args {
    #[arg(long, default_value = "screen")]
    output: String,

    #[arg(long, default_value = "")]
    nip: String,

    #[arg(long = "given-name", default_value = "Eugeniusz")]
    given_name: String,

    #[arg(long = "surname", default_value = "Fakturowski")]
    surname: String,

    #[arg(long = "serial-prefix", default_value = "EUGEPL")]
    serial_prefix: String,

    #[arg(long = "common-name", default_value = "E F")]
    common_name: String,

    #[arg(long, default_value = "EUGEPL")]
    organization: String,

    #[arg(long = "out-dir", default_value = ".")]
    out_dir: String,
}

fn generate_random_nip() -> String {
    let mut rng = rand::thread_rng();
    let digits = Uniform::from(0..10);
    (0..10).map(|_| rng.sample(&digits).to_string()).collect()
}

fn main() -> ExitCode {
    let args = Args::parse();

    let _output_mode = if args.output.to_lowercase() == "file" { "file" } else { "screen" };

    let _nip = if args.nip.trim().is_empty() {
        let rnd = generate_random_nip();
        println!("[1] Przygotowanie NIP...");
        println!("    NIP: {} (losowy)", rnd);
        rnd
    } else {
        println!("[1] Przygotowanie NIP...");
        println!("    NIP: {} (z parametru)", args.nip.trim());
        args.nip.trim().to_string()
    };

    println!("[1.1] Parametry subjectu certyfikatu:");
    println!("    GivenName: {}", &args.given_name);
    println!("    Surname: {}", &args.surname);
    println!("    Serial prefix: {}", &args.serial_prefix);
    println!("    Organization: {}", &args.organization);
    println!("    CommonName: {}", &args.common_name);

    println!("[1.2] Pobieranie AuthChallenge z KSeF...");
    let client = KsefClient::new();
    match client.get_auth_challenge() {
        Ok(ch) => println!("    AuthChallenge: {} (timestamp: {}, ts_ms: {})", ch.challenge, ch.timestamp, ch.timestamp_ms),
        Err(e) => println!("    Nie udało się pobrać AuthChallenge: {}", e),
    }

    println!("Zakończono pomyślnie.");
    ExitCode::SUCCESS
}
