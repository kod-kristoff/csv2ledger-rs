use std::error::Error as StdError;
use std::io;

pub mod process;
pub mod swedbank;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    CsvError(csv::Error),
    JsonError(serde_json::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<csv::Error> for Error {
    fn from(err: csv::Error) -> Self {
        Self::CsvError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::JsonError(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            &Self::IoError(err) => write!(f, "{}", err),
            &Self::CsvError(err) => write!(f, "{}", err),
            &Self::JsonError(err) => write!(f, "{}", err),
        }
    }
}
impl StdError for Error {}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
