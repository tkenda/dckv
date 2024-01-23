use dckv_parser::DBHandlerActions;

mod error;
mod rocks;

pub use error::DatabaseError;
pub use rocks::{RocksDB, RocksDBOpts};

pub type Result<T> = std::result::Result<T, DatabaseError>;