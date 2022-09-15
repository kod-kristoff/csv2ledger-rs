use chrono::NaiveDate;
use csv2ledger::{Entry, Transaction};
use env_logger::Env;
use rust_decimal::Decimal;
use std::fs;
use std::io::Read;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct SwedbankRow {
    clearingnummer: u32,
    kontonummer: u32,
    valuta: String,
    bokföringsdag: NaiveDate,
    transaktionsdag: NaiveDate,
    referens: String,
    beskrivning: String,
    belopp: Decimal,
    #[serde(alias = "Bokfört saldo")]
    bokfört_saldo: Decimal,
}

impl From<SwedbankRow> for Entry {
    fn from(row: SwedbankRow) -> Entry {
        let mut transactions = Vec::new();
        transactions.push(Transaction::new(
            format!(
                "Tillgång:Bank:Swedbank:{}:{}",
                row.clearingnummer, row.kontonummer
            ),
            row.belopp,
            row.valuta.clone(),
        ));
        let account = if row.belopp < Decimal::from(0) {
            format!("Kostnad:Import:{}", row.beskrivning)
        } else {
            format!("Inkomst:Import:{}", row.beskrivning)
        };
        transactions.push(Transaction::new(account, -row.belopp, row.valuta));
        let entry = Entry::new(row.transaktionsdag, row.beskrivning, transactions)
            .with_comment(format!("referens: {}", row.referens));
        if row.transaktionsdag != row.bokföringsdag {
            entry.with_secondary_date(row.bokföringsdag)
        } else {
            entry
        }
    }
}
fn write_row(row: Entry) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", row);
    Ok(())
}

fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    println!("args = {:?}", args);
    println!("args.len() = {}", args.len());
    let data_read = decode_data(&args[1])?;
    let mut rdr = csv::Reader::from_reader(&data_read[..]);
    for result in rdr.deserialize() {
        let record: SwedbankRow = result?;
        write_row(record.into())?;
    }
    Ok(())
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("trace"))
        .format_timestamp(None)
        .init();
    if let Err(err) = try_main() {
        eprintln!("error: {:?}", err);
        std::process::exit(1);
    }
}

fn decode_data(data: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut file = fs::File::open(data)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let (decoded_buffer, _, errors) = encoding_rs::WINDOWS_1252.decode(buffer.as_slice());
    if errors {
        log::warn!("Buffer contained malformed chars");
    }
    Ok(decoded_buffer.as_bytes().to_vec())
}
