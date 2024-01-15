use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Invalid preamble.")]
    InvalidPreamble,

    #[error("{0}")]
    IOError(#[from] std::io::Error),

    #[error("{0}")]
    DBError(String),
}
