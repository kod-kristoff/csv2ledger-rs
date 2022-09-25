use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, Command};
use csv2ledger::{process::Record2Entry, Error};
use env_logger::Env;
use eyre::WrapErr;
use std::fs;
use std::io::Read;

fn try_main() -> Result<(), eyre::Report> {
    let args = cli().get_matches();
    let csv_path = args
        .get_one::<String>("input")
        .expect("`input` is required");
    let output = args
        .get_one::<String>("output")
        .expect("`output` is required");
    let config_path = args.get_one::<String>("config");
    let mut fp_out = fs::File::create(output)?;
    let data_read = decode_data(csv_path)?;
    // let mut rdr = csv::Reader::from_reader(&data_read[..]);
    let processor = match config_path {
        Some(path) => {
            let mut config_fp = fs::File::open(path)
                .wrap_err_with(|| format!("Failed to read config from '{}'", path))?;
            Record2Entry::from_reader(&mut config_fp)?
        }
        None => {
            if let Some(proj_dirs) = directories::ProjectDirs::from("ml", "kristoff", crate_name!())
            {
                let path = proj_dirs.config_dir().join("csv2ledger.config.json");
                match fs::File::open(&path) {
                    Ok(mut fp) => Record2Entry::from_reader(&mut fp)?,
                    Err(err) => {
                        log::info!("Failed read config from '{}", path.display());
                        log::info!(" Caused by: '{}", err);
                        Record2Entry::new()
                    }
                }
            } else {
                Record2Entry::new()
            }
        }
    };
    processor.csv_to_ledger(&data_read[..], &mut fp_out)?;
    // for result in rdr.deserialize() {
    //     let record: SwedbankRow = result?;
    //     write_row(record.into())?;
    // }
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

fn decode_data(data: &str) -> Result<Vec<u8>, Error> {
    let mut file = fs::File::open(data)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let (decoded_buffer, _, errors) = encoding_rs::WINDOWS_1252.decode(buffer.as_slice());
    if errors {
        log::warn!("Buffer contained malformed chars");
    }
    Ok(decoded_buffer.as_bytes().to_vec())
}

fn cli() -> Command<'static> {
    Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("config")
                .long("config")
                .short('c')
                .takes_value(true),
        )
        .arg(Arg::new("input").takes_value(true).required(true))
        .arg(Arg::new("output").takes_value(true).required(true))
}
