use crate::swedbank::SwedbankRow;
use rledger::Entry;
use std::io;

pub fn csv_to_ledger(
    reader: Box<dyn io::Read>,
    writer: &mut dyn io::Write,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_reader(reader);
    for result in rdr.deserialize() {
        let record: SwedbankRow = result?;
        let entry: Entry = record.into();
        writeln!(writer, "{}", entry)?;
    }
    Ok(())
}
