use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("{0}")]
    DatabaseError(String),
}
