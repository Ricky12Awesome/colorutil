use palette::rgb::FromHexError;
use thiserror::Error;

pub mod parse;
pub mod color;

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    StdIo(#[from] std::io::Error),

    #[error(transparent)]
    FromHexError(#[from] FromHexError),

    #[error("Failed to parse")]
    FailedToParse,
    
    #[error("Failed to parse format")]
    FailedToParseFormat,
    
    #[error("Failed to parse color")]
    FailedToParseColor,
    
    #[error("Failed to parse params {0}")]
    FailedToParseColorParams(String),
}
