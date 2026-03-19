use chrono::{Duration, Utc};
use ksef_client::auth::SubjectIdentifierType;
use ksef_client::prelude::{
    ContextIdentifier, ContextIdentifierType, Environment, KsefAuth, KsefClient,
};
use rand::random_range;

#[allow(dead_code)]
pub async fn generate_random_nip() -> String {
    loop {
        let mut digits: Vec<u8> = (0..9).map(|_| random_range(0..10) as u8).collect();
        // Use a valid Tax Office prefix (e.g. 526 for Warszawa-Mokotów) to pass validation
        digits[0] = 5;
        digits[1] = 2;
        digits[2] = 6;

        let weights = [6, 5, 7, 2, 3, 4, 5, 6, 7];
        let sum: u32 = digits
            .iter()
            .zip(weights.iter())
            .map(|(d, w)| (*d as u32) * (*w as u32))
            .sum();

        let checksum = sum % 11;
        if checksum != 10 {
            digits.push(checksum as u8);
            return digits.iter().map(|d| d.to_string()).collect();
        }
    }
}

#[allow(dead_code)]
pub async fn authorize_client() -> KsefClient {
    let nip = "5261234567";
    let context = ContextIdentifier {
        id_type: ContextIdentifierType::Nip,
        value: nip.to_string(),
    };
    let mut client = KsefClient::new(Environment::Test, context);

    let given_name = "Eugeniusz";
    let surname = "Fakturowski";
    let serial_prefix = "TINPL";
    let common_name = "Eugeniusz Fakturowski";

    let auth_token_request = client
        .get_auth_token_request(SubjectIdentifierType::CertificateSubject)
        .await
        .expect("Failed to get auth token request");

    let unsigned_xml = auth_token_request.to_xml();

    client
        .xades
        .gen_selfsign_cert(given_name, surname, serial_prefix, &nip, common_name)
        .expect("Failed to generate self-signed certificate");

    let signed_xml = client
        .xades
        .sign(&unsigned_xml)
        .expect("Failed to sign XML");

    match client.authenticate_by_xades_signature(signed_xml).await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Authentication request submission failed: {:?}", e);
            panic!("Failed to authenticate: {:?}", e);
        }
    }

    match client.get_auth_status().await {
        Ok(true) => {}
        Ok(false) => {
            eprintln!("Authentication status check failed: Authentication not successful");
            panic!("Authentication not successful");
        }
        Err(e) => {
            panic!("Error checking auth status: {:?}", e);
        }
    }

    let _ = client.get_access_token().await;

    client
}

#[allow(dead_code)]
pub async fn generate_fa2_invoice(issuer_nip: &str) -> String {
    let number: u16 = random_range(10000..=65535);
    let inv_number = format!("{}", number);

    let now = Utc::now();
    let date = now.format("%Y-%m-%d").to_string();
    let date_plus_1 = (now + Duration::days(1)).format("%Y-%m-%d").to_string();
    let date_iso = now.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

    let xml = r#"
    <?xml version="1.0" encoding="utf-8"?>
    <Faktura xmlns="http://crd.gov.pl/wzor/2025/06/25/13775/" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    <Naglowek>
        <KodFormularza wersjaSchemy="1-0E" kodSystemowy="FA (3)">FA</KodFormularza>
        <WariantFormularza>3</WariantFormularza>
        <DataWytworzeniaFa>#DataWytworzeniaFa#</DataWytworzeniaFa>
        <SystemInfo>invoice-gen-rs</SystemInfo>
    </Naglowek>
    <Podmiot1>
        <DaneIdentyfikacyjne>
        <NIP>#nip#</NIP>
        <Nazwa>Acme Widgets Sp. z o.o.</Nazwa>
        </DaneIdentyfikacyjne>
        <Adres>
        <KodKraju>PL</KodKraju>
        <AdresL1>ul. Przykładowa 1, 00-001 Warszawa</AdresL1>
        </Adres>
        <StatusInfoPodatnika>2</StatusInfoPodatnika>
    </Podmiot1>
    <Podmiot2>
        <DaneIdentyfikacyjne>
        <NIP>9876543210</NIP>
        <Nazwa>Klient S.A.</Nazwa>
        </DaneIdentyfikacyjne>
        <Adres>
        <KodKraju>PL</KodKraju>
        <AdresL1>ul. Kupiecka 5, 00-100 Warszawa</AdresL1>
        </Adres>
        <JST>2</JST>
        <GV>2</GV>
    </Podmiot2>
    <Podmiot3>
        <DaneIdentyfikacyjne>
        <NIP>9876543210</NIP>
        <Nazwa>Podmiot3-Example</Nazwa>
        </DaneIdentyfikacyjne>
        <Rola>5</Rola>
    </Podmiot3>
    <PodmiotUpowazniony>
        <DaneIdentyfikacyjne>
        <NIP>9876543210</NIP>
        <Nazwa>Upowazniony-Example</Nazwa>
        </DaneIdentyfikacyjne>
        <Adres>
        <KodKraju>PL</KodKraju>
        <AdresL1>Upoważniony ul 1, 00-000 City</AdresL1>
        </Adres>
        <RolaPU>2</RolaPU>
    </PodmiotUpowazniony>
    <Fa>
        <KodWaluty>EUR</KodWaluty>
        <P_1>#DataDostawy#</P_1>
        <P_2>#invoice_number#</P_2>
        <P_6>#DataFaktury#</P_6>
        <P_13_1>30.00</P_13_1>
        <P_14_1>6.90</P_14_1>
        <P_15>36.90</P_15>
        <Adnotacje>
        <P_16>2</P_16>
        <P_17>2</P_17>
        <P_18>2</P_18>
        <P_18A>2</P_18A>
        <Zwolnienie>
            <P_19N>1</P_19N>
        </Zwolnienie>
        <NoweSrodkiTransportu>
            <P_22N>1</P_22N>
        </NoweSrodkiTransportu>
        <P_23>2</P_23>
        <PMarzy>
            <P_PMarzyN>1</P_PMarzyN>
        </PMarzy>
        </Adnotacje>
        <RodzajFaktury>VAT</RodzajFaktury>
        <FaWiersz>
        <NrWierszaFa>1</NrWierszaFa>
        <UU_ID>uuid-subj3-1</UU_ID>
        <P_7>Produkt z GTU i procedurą</P_7>
        <Indeks>IDX-GTU-1</Indeks>
        <P_8A>szt.</P_8A>
        <P_8B>2</P_8B>
        <P_9A>15.00</P_9A>
        <P_11>30.00</P_11>
        <P_12>23</P_12>
        <GTU>GTU_01</GTU>
        <Procedura>B_SPV</Procedura>
        </FaWiersz>
    </Fa>
    </Faktura>
    "#.replace("#nip#", issuer_nip)
    .replace("#invoice_number#", &inv_number)
    .replace("#DataWytworzeniaFa#", &date_iso)
    .replace("#DataDostawy#", &date) // P_1
    .replace("#DataFaktury#", &date) // P_6
    .replace("#DataZaplaty#", &date_plus_1); // DataZaplaty

    xml.trim().to_string()
}

#[allow(dead_code)]
pub fn generate_ec_private_key_pem() -> (String, openssl::pkey::PKey<openssl::pkey::Private>) {
    use openssl::ec::EcKey;
    use openssl::nid::Nid;
    use openssl::pkey::PKey;

    let ec_key =
        EcKey::generate(&openssl::ec::EcGroup::from_curve_name(Nid::X9_62_PRIME256V1).unwrap())
            .expect("ec gen");
    let pkey = PKey::from_ec_key(ec_key).expect("pkey ec");
    let pem = String::from_utf8(pkey.private_key_to_pem_pkcs8().unwrap()).unwrap();
    (pem, pkey)
}

#[allow(dead_code)]
pub fn generate_rsa_private_key_pem() -> (String, openssl::pkey::PKey<openssl::pkey::Private>) {
    use openssl::pkey::PKey;
    use openssl::rsa::Rsa;

    let rsa = Rsa::generate(2048).expect("rsa gen");
    let pkey = PKey::from_rsa(rsa).expect("pkey rsa");
    let pem = String::from_utf8(pkey.private_key_to_pem_pkcs8().unwrap()).unwrap();
    (pem, pkey)
}
