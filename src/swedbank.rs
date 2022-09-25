use chrono::NaiveDate;
use rledger::{Entry, Transaction};
use rust_decimal::Decimal;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct SwedbankRow {
    pub clearingnummer: u32,
    pub kontonummer: u32,
    pub valuta: String,
    pub bokföringsdag: NaiveDate,
    pub transaktionsdag: NaiveDate,
    pub referens: String,
    pub beskrivning: String,
    pub belopp: Decimal,
    #[serde(alias = "Bokfört saldo")]
    pub bokfört_saldo: Decimal,
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
