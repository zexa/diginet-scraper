use log::SetLoggerError;
use std::fmt::{Display, Formatter, Result};
use std::error::Error;
use url::ParseError;

#[derive(Debug)]
pub struct DiginetError {}

impl Display for DiginetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "DiginetScraper error.")
    }
}

impl Error for DiginetError {}

impl From<()> for DiginetError {
    fn from(_: ()) -> Self {
        Self {}
    }
}

impl From<anyhow::Error> for DiginetError {
    fn from(_: anyhow::Error) -> Self {
        Self {}
    }
}

impl From<ParseError> for DiginetError {
    fn from(_: ParseError) -> Self {
        Self {}
    }
}

impl From<SetLoggerError> for DiginetError {
    fn from(_: SetLoggerError) -> Self {
        Self {}
    }
}
