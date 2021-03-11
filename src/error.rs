use log::SetLoggerError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use std::option::NoneError;
use url::ParseError;

#[derive(Debug)]
pub struct DiginetError {}

impl Display for DiginetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "DiginetScraper error.")
    }
}

impl Error for DiginetError {}

impl From<NoneError> for DiginetError {
    fn from(_: NoneError) -> Self {
        Self {}
    }
}

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

impl From<url::ParseError> for DiginetError {
    fn from(_: url::ParseError) -> Self {
        Self {}
    }
}

impl From<SetLoggerError> for DiginetError {
    fn from(_: SetLoggerError) -> Self {
        Self {}
    }
}
