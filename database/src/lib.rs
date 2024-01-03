mod error;
mod rocks;

pub use error::DatabaseError;
pub use rocks::{RocksDB, RocksDBConfig};

pub type Result<T> = std::result::Result<T, DatabaseError>;

pub trait DBActions<DB: DBActions<DB, DBConfig>, DBConfig> {
    fn open(config: &DBConfig, group: &str) -> Result<DB>;
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;
    fn put(&self, key: &[u8], value: &[u8]) -> Result<()>;
    fn delete(&self, key: &[u8]) -> Result<()>;
}
