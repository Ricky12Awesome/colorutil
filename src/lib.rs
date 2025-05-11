use std::path::PathBuf;
use palette::rgb::FromHexError;
use thiserror::Error;

pub mod color;
pub mod config;
pub mod parse;

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    StdIo(#[from] std::io::Error),

    #[error(transparent)]
    Toml(#[from] toml::de::Error),
    
    #[error("cannot find config path")]
    NoConfigPath,

    #[error("cannot find palette {0} in {1}")]
    NoInherit(String, String),

    #[error("cannot find palette {0}")]
    NoPalette(String),

    #[error("invalid config")]
    InvalidConfig,
    
    #[error("not a file: {0}")]
    NotFile(PathBuf),

    #[error(transparent)]
    FromHexError(#[from] FromHexError),
    
    #[error("Failed to parse: {0}")]
    FailedToParseValue(String),

    #[error("Failed to find suffix starting from: {0}")]
    FailedToFindSuffix(usize),

    #[error("Failed to get color: {0}")]
    FailedToGetColor(String),

    #[error("Failed to parse format: {0}")]
    FailedToParseFormat(String),

    #[error("Failed to parse color: {0}")]
    FailedToParseColor(String),

    #[error("Failed to parse params: {0}")]
    FailedToParseColorParams(String),
}
