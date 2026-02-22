mod common;

use chrono::{Duration, Utc};
use ksef_client::OpenOnlineSessionRequestBuilder;
use rand::random_range;

#[test]
fn test_open_online_session() {
    let client = common::authorize_client();

    let encryption_data = client
        .generate_encryption_data()
        .expect("Failed to generate encryption data");

    let request = OpenOnlineSessionRequestBuilder::new()
        .with_encryption(
            &encryption_data.encrypted_symmetric_key,
            &encryption_data.initialization_vector,
        )
        .build()
        .expect("Failed to build OpenOnlineSessionRequest");

    let response = client
        .open_online_session(request)
        .expect("Failed to open online session");

    println!("Opened online session: {:?}", response);

    assert!(
        !response.reference_number.is_empty(),
        "Session reference number should not be empty"
    );

    let issuer_nip = "5264567890";
    let invoice_xml = generate_fa2_invoice(issuer_nip);

    let send_result = client.send_invoice(
        &response.reference_number,
        invoice_xml.as_bytes(),
        &encryption_data,
    );

    let invoice_reference_number = match send_result {
        Ok(resp) => resp.reference_number,
        Err(e) => {
            panic!("Invoice send failed: {:?}", e);
        }
    };

    let status = client
        .get_invoice_status(&response.reference_number, &invoice_reference_number)
        .expect("Failed to get invoice status");

    println!("Final Invoice status: {:#?}", status);

    assert!(status.invoice_status.code == 200);
}

fn generate_fa2_invoice(issuer_nip: &str) -> String {
    let number: u16 = random_range(10000..=65535);
    let inv_number = format!("{}", number);

    let now = Utc::now();
    let date = now.format("%Y-%m-%d").to_string();
    let date_plus_1 = (now + Duration::days(1)).format("%Y-%m-%d").to_string();
    let date_iso = now.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

    let xml = r#"
        <?xml version="1.0" encoding="utf-8"?>
        <Faktura xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:etd="http://crd.gov.pl/xml/schematy/2020/10/08/eDokumenty" xmlns="http://crd.gov.pl/wzor/2023/06/29/12648/">
            <Naglowek>
          		<KodFormularza kodSystemowy="FA (2)" wersjaSchemy="1-0E">FA</KodFormularza>
          		<WariantFormularza>2</WariantFormularza>
          		<DataWytworzeniaFa>#DataWytworzeniaFa#</DataWytworzeniaFa>
          		<SystemInfo>Generator danych</SystemInfo>
           	</Naglowek>
           	<Podmiot1>
          		<DaneIdentyfikacyjne>
         			<NIP>#nip#</NIP>
         			<Nazwa>Baranowski, Kucharski and Krupa</Nazwa>
          		</DaneIdentyfikacyjne>
          		<Adres>
         			<KodKraju>PL</KodKraju>
         			<AdresL1>al. Stachowicz 630</AdresL1>
         			<AdresL2>13-903 Ostrołęka</AdresL2>
          		</Adres>
          		<DaneKontaktowe>
         			<Email>Konstancja7@example.net</Email>
         			<Telefon>24-106-12-95</Telefon>
          		</DaneKontaktowe>
           	</Podmiot1>
           	<Podmiot2>
          		<DaneIdentyfikacyjne>
         			<NIP>7352765225</NIP>
         			<Nazwa>Cichy LLC</Nazwa>
          		</DaneIdentyfikacyjne>
          		<Adres>
         			<KodKraju>PL</KodKraju>
         			<AdresL1>ul. Stolarczyk 0260</AdresL1>
         			<AdresL2>62-075 Kamieńsk</AdresL2>
          		</Adres>
          		<DaneKontaktowe>
         			<Email>Ludwika.Kaczmarski60@hotmail.com</Email>
         			<Telefon>23-210-25-56</Telefon>
          		</DaneKontaktowe>
          		<NrKlienta>KL-9722</NrKlienta>
           	</Podmiot2>
           	<Fa>
          		<KodWaluty>PLN</KodWaluty>
          		<P_1>#DataDostawy#</P_1>
          		<P_1M>Ostrów Lubelski</P_1M>
          		<P_2>FA/GRQMB-#invoice_number#/05/2025</P_2>
          		<P_6>#DataFaktury#</P_6>
          		<P_13_1>35260.63</P_13_1>
          		<P_14_1>8109.94</P_14_1>
          		<P_15>43370.57</P_15>
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
         			<UU_ID>47f6fb60-6a23-6e34-2e03-273f7a70accd</UU_ID>
         			<P_7>Sleek Cotton Car</P_7>
         			<P_8A>szt.</P_8A>
         			<P_8B>1</P_8B>
         			<P_9A>444.63</P_9A>
         			<P_11>444.63</P_11>
         			<P_12>23</P_12>
          		</FaWiersz>
          		<FaWiersz>
         			<NrWierszaFa>2</NrWierszaFa>
         			<UU_ID>62059164-04c5-22fd-1342-be358f55f6fb</UU_ID>
         			<P_7>Unbranded Wooden Chicken</P_7>
         			<P_8A>szt.</P_8A>
         			<P_8B>9</P_8B>
         			<P_9A>887.59</P_9A>
         			<P_11>7988.31</P_11>
         			<P_12>23</P_12>
          		</FaWiersz>
          		<FaWiersz>
         			<NrWierszaFa>3</NrWierszaFa>
         			<UU_ID>df0332e4-973c-c356-76f2-25827b440d01</UU_ID>
         			<P_7>Unbranded Soft Pants</P_7>
         			<P_8A>szt.</P_8A>
         			<P_8B>3</P_8B>
         			<P_9A>618.56</P_9A>
         			<P_11>1855.68</P_11>
         			<P_12>23</P_12>
          		</FaWiersz>
          		<FaWiersz>
         			<NrWierszaFa>4</NrWierszaFa>
         			<UU_ID>75a3496d-ae4f-3368-c9ac-f3feb5dc3e3a</UU_ID>
         			<P_7>Gorgeous Soft Towels</P_7>
         			<P_8A>szt.</P_8A>
         			<P_8B>10</P_8B>
         			<P_9A>650.29</P_9A>
         			<P_11>6502.90</P_11>
         			<P_12>23</P_12>
          		</FaWiersz>
          		<FaWiersz>
         			<NrWierszaFa>5</NrWierszaFa>
         			<UU_ID>6a38a9d0-3a1f-b148-2e98-7271c5631b93</UU_ID>
         			<P_7>Practical Wooden Chair</P_7>
         			<P_8A>szt.</P_8A>
         			<P_8B>7</P_8B>
         			<P_9A>590.26</P_9A>
         			<P_11>4131.82</P_11>
         			<P_12>23</P_12>
          		</FaWiersz>
          		<FaWiersz>
         			<NrWierszaFa>6</NrWierszaFa>
         			<UU_ID>15e20821-981b-4e90-36ca-c856beb3bd27</UU_ID>
         			<P_7>Sleek Cotton Table</P_7>
         			<P_8A>szt.</P_8A>
         			<P_8B>6</P_8B>
         			<P_9A>569.78</P_9A>
         			<P_11>3418.68</P_11>
         			<P_12>23</P_12>
          		</FaWiersz>
          		<FaWiersz>
         			<NrWierszaFa>7</NrWierszaFa>
         			<UU_ID>5955219c-49e1-465a-3268-e5897e5005c9</UU_ID>
         			<P_7>Licensed Steel Chicken</P_7>
         			<P_8A>szt.</P_8A>
         			<P_8B>5</P_8B>
         			<P_9A>967.18</P_9A>
         			<P_11>4835.90</P_11>
         			<P_12>23</P_12>
          		</FaWiersz>
          		<FaWiersz>
         			<NrWierszaFa>8</NrWierszaFa>
         			<UU_ID>f5a983c6-9897-98c1-1ebb-3e7a73d8470f</UU_ID>
         			<P_7>Intelligent Rubber Computer</P_7>
         			<P_8A>szt.</P_8A>
         			<P_8B>10</P_8B>
         			<P_9A>207.45</P_9A>
         			<P_11>2074.50</P_11>
         			<P_12>23</P_12>
          		</FaWiersz>
          		<FaWiersz>
         			<NrWierszaFa>9</NrWierszaFa>
         			<UU_ID>2673ccc8-07d7-a149-a73f-255799eeb523</UU_ID>
         			<P_7>Practical Plastic Computer</P_7>
         			<P_8A>szt.</P_8A>
         			<P_8B>5</P_8B>
         			<P_9A>8.59</P_9A>
         			<P_11>42.95</P_11>
         			<P_12>23</P_12>
          		</FaWiersz>
          		<FaWiersz>
         			<NrWierszaFa>10</NrWierszaFa>
         			<UU_ID>043ee5de-7b41-cadb-8c2a-207c771a7394</UU_ID>
         			<P_7>Refined Steel Chair</P_7>
         			<P_8A>szt.</P_8A>
         			<P_8B>6</P_8B>
         			<P_9A>192.85</P_9A>
         			<P_11>1157.10</P_11>
         			<P_12>23</P_12>
          		</FaWiersz>
          		<FaWiersz>
         			<NrWierszaFa>11</NrWierszaFa>
         			<UU_ID>486433c8-18c2-2a8f-ca7d-a0e75ba51a1c</UU_ID>
         			<P_7>Ergonomic Wooden Mouse</P_7>
         			<P_8A>szt.</P_8A>
         			<P_8B>4</P_8B>
         			<P_9A>640.50</P_9A>
         			<P_11>2562.00</P_11>
         			<P_12>23</P_12>
          		</FaWiersz>
          		<FaWiersz>
         			<NrWierszaFa>12</NrWierszaFa>
         			<UU_ID>31227874-74eb-bff0-3209-1a0f2afe192f</UU_ID>
         			<P_7>Licensed Wooden Cheese</P_7>
         			<P_8A>szt.</P_8A>
         			<P_8B>8</P_8B>
         			<P_9A>30.77</P_9A>
         			<P_11>246.16</P_11>
         			<P_12>23</P_12>
          		</FaWiersz>
          		<Platnosc>
         			<Zaplacono>1</Zaplacono>
         			<DataZaplaty>#DataZaplaty#</DataZaplaty>
         			<FormaPlatnosci>4</FormaPlatnosci>
          		</Platnosc>
           	</Fa>
        </Faktura>
    "#
    .replace("#nip#", issuer_nip)
    .replace("#invoice_number#", &inv_number)
    .replace("#DataWytworzeniaFa#", &date_iso)
    .replace("#DataDostawy#", &date) // P_1
    .replace("#DataFaktury#", &date) // P_6
    .replace("#DataZaplaty#", &date_plus_1); // DataZaplaty

    xml.trim().to_string()
}
