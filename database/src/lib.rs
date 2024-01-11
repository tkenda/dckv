use dckv_parser::DBActions;

mod error;
mod rocks;

pub use error::DatabaseError;
pub use rocks::{RocksDB, RocksDBConfig};

pub type Result<T> = std::result::Result<T, DatabaseError>;