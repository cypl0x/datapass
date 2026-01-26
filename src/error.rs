use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatapassError {
    #[error("Failed to fetch data: {0}")]
    FetchError(#[from] reqwest::Error),

    #[error("Failed to parse HTML: {0}")]
    ParseError(String),

    #[error("Data not found in HTML: {0}")]
    DataNotFound(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse float: {0}")]
    FloatParseError(#[from] std::num::ParseFloatError),
}

pub type Result<T> = std::result::Result<T, DatapassError>;
