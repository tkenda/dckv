use thiserror::Error;

use dckv_parser::{Category, ParserError};

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("RocksDB error: {0}")]
    RocksDBError(#[from] rocksdb::Error),

    #[error("Can't create handle for category: {0}.")]
    CategoryHandleError(Category),
}

impl From<DatabaseError> for ParserError {
    fn from(value: DatabaseError) -> Self {
        ParserError::DatabaseError(value.to_string())
    }
}
