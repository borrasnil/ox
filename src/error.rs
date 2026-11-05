use thiserror::Error;

#[derive(Error, Debug)]
pub enum OxError {
    #[error("Parsing failed")]
    ParsingFailed,

    #[error("Invalid format")]
    InvalidFormat,
}
