use crate::swedbank::SwedbankRow;
use rledger::{Entry, Transaction};
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::io::Read;
use std::{fs, io};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Record2Entry {
    general: HashMap<String, String>,
    descriptions: HashMap<String, String>,
    accounts: HashMap<u32, HashMap<u32, String>>,
}

impl Record2Entry {
    pub fn new() -> Self {
        Self {
            general: HashMap::new(),
            accounts: HashMap::new(),
            descriptions: HashMap::new(),
        }
    }

    pub fn from_path<P: AsRef<std::path::Path>>(
        path: P,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = fs::File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let r: Self = serde_json::from_str(&data)?;
        Ok(r)
    }
    pub fn row2entry(&self, row: SwedbankRow) -> Entry {
        let mut transactions = Vec::new();
        transactions.push(
            Transaction::new(
                format!(
                    "{}:Bank:{}",
                    self.assets_name(),
                    self.account_name(row.clearingnummer, row.kontonummer,)
                ),
                row.belopp,
                row.valuta.clone(),
            )
            .assertion(row.bokfört_saldo, row.valuta.clone()),
        );
        let account_from_desc = self.account_from_desc(&row.beskrivning);
        let account = if row.belopp < Decimal::from(0) {
            format!("{}:{}", self.expenses_name(), account_from_desc)
        } else {
            format!("{}:{}", self.income_name(), account_from_desc)
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

    pub fn assets_name(&self) -> &str {
        self.general
            .get("Assets")
            .as_ref()
            .map_or("Assets", |x| &**x)
    }

    pub fn expenses_name(&self) -> &str {
        self.general
            .get("Expenses")
            .as_ref()
            .map_or("Expenses", |x| &**x)
    }

    pub fn income_name(&self) -> &str {
        self.general
            .get("Income")
            .as_ref()
            .map_or("Income", |x| &**x)
    }

    pub fn account_from_desc(&self, desc: &str) -> String {
        self.descriptions
            .get(desc)
            .map_or_else(|| format!("Import:{}", desc), String::clone)
    }

    pub fn account_name(&self, cl_nr: u32, acc_nr: u32) -> String {
        self.accounts
            .get(&cl_nr)
            .map(|m| m.get(&acc_nr))
            .flatten()
            .map_or_else(|| format!("{}:{}", cl_nr, acc_nr), String::clone)
    }

    pub fn csv_to_ledger(
        &self,
        // reader: Box<dyn io::Read>,
        reader: &[u8],
        writer: &mut dyn io::Write,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut rdr = csv::Reader::from_reader(reader);
        for result in rdr.deserialize() {
            // let record: SwedbankRow = result?;
            // let entry: Entry = record.into();
            let entry = self.row2entry(result?);
            writeln!(writer, "{}", entry)?;
        }
        Ok(())
    }
}
